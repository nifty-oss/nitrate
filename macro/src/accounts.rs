use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, DeriveInput, Error, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Result};

// Constants for the account attribute.
const ACCOUNT_TOKEN: &str = "account";

// Constants for the account attribute name property.
const NAME_TOKEN: &str = "name";

// Constants for the account attribute optional property.
const OPTIONAL_TOKEN: &str = "optional";

/// Generates the account structs for each variant of the enum.
pub fn generate_accounts(ast: DeriveInput) -> Result<TokenStream> {
    // parses each variant of the enum:
    //   1. extracts the account name and optional "status"
    //   2. generate the account struct
    let instructions = if let syn::Data::Enum(syn::DataEnum { ref variants, .. }) = ast.data {
        let mut instructions = Vec::new();

        for v in variants {
            let mut instruction = Instruction {
                name: v.ident.to_string(),
                ..Default::default()
            };

            for a in &v.attrs {
                let syn::Attribute {
                    path: syn::Path { segments, .. },
                    ..
                } = &a;

                for path in segments {
                    let ident = path.ident.to_string();

                    if ident == ACCOUNT_TOKEN {
                        let meta_tokens = a
                            .parse_meta()
                            .map_err(|_error| Error::new_spanned(a, "#[account] is required"))?;

                        let nested_meta = if let Meta::List(MetaList { nested, .. }) = &meta_tokens
                        {
                            nested
                        } else {
                            return Err(Error::new_spanned(a, "#[account] is required"));
                        };

                        let mut property: (Option<String>, Option<String>) = (None, None);

                        for element in nested_meta {
                            match element {
                                NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                    path,
                                    lit,
                                    ..
                                })) => {
                                    let ident = path.get_ident();
                                    if let Some(ident) = ident {
                                        if *ident == NAME_TOKEN {
                                            let token = match lit {
                                                Lit::Str(lit) => {
                                                    lit.token().to_string().replace('\"', "")
                                                }
                                                _ => {
                                                    return Err(Error::new_spanned(
                                                        ident,
                                                        "invalid value for \'name\' property",
                                                    ));
                                                }
                                            };
                                            property.0 = Some(token);
                                        }
                                    }
                                }
                                NestedMeta::Meta(Meta::Path(path)) => {
                                    let name = path.get_ident().map(|x| x.to_string());
                                    if let Some(name) = name {
                                        if name == OPTIONAL_TOKEN {
                                            property.1 = Some(name);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        instruction.accounts.push(Account {
                            name: property.0.ok_or(Error::new_spanned(
                                a,
                                "account \'name\' property is required",
                            ))?,
                            optional: property.1.is_some(),
                        });
                    }
                }
            }

            instructions.push(instruction);
        }

        instructions
    } else {
        return Err(Error::new_spanned(&ast, "no enum variants found"));
    };

    Ok(render_accounts(&instructions))
}

/// Renders a struct for each enum variant (instruction).
fn render_accounts(instructions: &[Instruction]) -> TokenStream {
    let instruction_structs = instructions.iter().map(|instruction| {
        let name = syn::parse_str::<syn::Ident>(&instruction.name).unwrap();
        // fields
        let struct_fields = instruction.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            if account.optional {
                quote! {
                    pub #account_name: Option<&'a nitrate::program::AccountInfo>
                }
            } else {
                quote! {
                    pub #account_name:&'a nitrate::program::AccountInfo
                }
            }
        });
        // initialization
        let account_fields = instruction.accounts.iter().enumerate().map(|(index, account)| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();

            if account.optional {
                quote! {
                    #account_name: if accounts[#index].key() == &crate::ID { None } else { Some(&accounts[#index]) }
                }
            } else {
                quote! {
                    #account_name: &accounts[#index]
                }
            }
        });
        // expected accounts
        let expected = instruction.accounts.len();

        quote! {
            pub struct #name<'a> {
                #(#struct_fields,)*
            }
            impl<'a> #name<'a> {
                #[inline(always)]
                pub fn context(accounts: &'a [nitrate::program::AccountInfo]) -> Result<Context<Self>, solana_program::program_error::ProgramError> {
                    if accounts.len() < #expected {
                        return Err(solana_program::program_error::ProgramError::NotEnoughAccountKeys);
                    }
                    Ok(Context {
                        accounts: Self {
                            #(#account_fields,)*
                        },
                    })
                }
            }
        }
    });

    quote! {
        #(#instruction_structs)*
    }
}

/// Internal representation of an instruction.
#[derive(Default)]
struct Instruction {
    pub name: String,
    pub accounts: Vec<Account>,
}

/// Internal representation of an account.
#[derive(Debug)]
struct Account {
    pub name: String,
    pub optional: bool,
}
