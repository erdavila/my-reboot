use std::io;

use ansi_term::Color;
use anyhow::Result;
use display_profile_lib::{Profile, SetProfileAction, get_profile, set_profile};

use crate::configuration::Configurer;
use crate::options_types::{LabeledProfile, ProfileId};
use crate::persist::configs::ConfigsWriter;
use crate::text::{IndentedBlockWriter, IndentedBlockWriterExt as _};

pub(crate) fn configure(configurer: &mut Configurer) -> Result<()> {
    let mut configurer = WindowsConfigurer::new(configurer)?;
    configurer.configure()?;
    configurer.finalize()?;
    Ok(())
}

macro_rules! print_error {
    ($($tt:tt)*) => {
        println!("{} {}", Color::Red.paint("Erro:"), format_args!($($tt)*))
    };
}

struct WindowsConfigurer<'a> {
    initial_profile: Profile,
    inner: &'a mut Configurer,
}

impl<'a> WindowsConfigurer<'a> {
    fn new(inner: &'a mut Configurer) -> Result<Self> {
        let initial_profile = get_profile()?;
        Ok(Self {
            initial_profile,
            inner,
        })
    }

    #[expect(clippy::similar_names)]
    fn configure(&mut self) -> Result<()> {
        println!(
            "Configuraremos perfis {} e {} de telas do Windows.",
            ProfileId::A,
            ProfileId::B
        );

        let (profile_a, profile_a_label) =
            self.configure_profile_and_label(ProfileId::A, accept_anything, accept_anything)?;

        let (profile_b, profile_b_label) = self.configure_profile_and_label(
            ProfileId::B,
            |profile| {
                if *profile == profile_a {
                    Err(format!(
                        "A configuração não pode ser igual à do perfil {}",
                        ProfileId::A
                    ))
                } else {
                    Ok(())
                }
            },
            |label| {
                if *label == profile_a_label {
                    Err(format!(
                        "O nome não pode ser igual ao do perfil {}",
                        ProfileId::A
                    ))
                } else {
                    Ok(())
                }
            },
        )?;

        println!();

        let mut writer = IndentedBlockWriter::from(io::stdout());
        writer.write_block("Resumo dos perfis:", |w| {
            w.write_profile_summary(
                LabeledProfile::new(ProfileId::A, &profile_a_label),
                &profile_a,
            )?;
            w.write_profile_summary(
                LabeledProfile::new(ProfileId::B, &profile_b_label),
                &profile_b,
            )?;
            w.write("")
        })?;

        self.configs_mut()
            .set_profile_configs(ProfileId::A, &profile_a_label, &profile_a)?;
        self.configs_mut()
            .set_profile_configs(ProfileId::B, &profile_b_label, &profile_b)?;

        Ok(())
    }

    fn current_profile(&self, id: ProfileId) -> Option<(String, Profile)> {
        self.configs().profile_configs(id).and_then(Result::ok)
    }

    fn configure_profile_and_label(
        &mut self,
        id: ProfileId,
        validate_profile: impl Fn(&Profile) -> Result<(), String>,
        validate_label: impl Fn(&str) -> Result<(), String>,
    ) -> Result<(Profile, String)> {
        let current = self.current_profile(id);
        let (current_label, current_profile) = current.unzip();

        let profile = self.configure_profile(
            id,
            current_label.as_deref().zip(current_profile),
            validate_profile,
        )?;
        let label = self.ask_label(id, current_label, validate_label)?;

        Ok((profile, label))
    }

    fn configure_profile(
        &mut self,
        id: ProfileId,
        current: Option<(&str, Profile)>,
        validate: impl Fn(&Profile) -> Result<(), String>,
    ) -> Result<Profile> {
        let current = current.filter(|(_label, profile)| validate(profile).is_ok());

        loop {
            println!();
            println!("Escolha uma das opções para configurar o perfil {id}:");
            println!("1. A configuração de tela atual corresponde ao perfil {id}");
            println!("2. Abrir as configurações de tela do Windows");
            if let Some((label, profile)) = &current {
                let mut writer = IndentedBlockWriter::from(io::stdout());
                writer.write_block("Tecle ENTER para manter a configuração", |w| {
                    w.write_profile_summary(format_args!("\"{label}\""), profile)
                })?;
            }

            match self.readline()?.as_str() {
                "1" => {
                    let profile = get_profile()?;
                    match validate(&profile) {
                        Ok(()) => {
                            self.restore_initial_profile()?;
                            return Ok(profile);
                        }
                        Err(msg) => print_error!("{msg}"),
                    }
                }
                "2" => open::that("ms-settings:display")?,
                "" if let Some((_label, profile)) = current => return Ok(profile),
                other => print_error!("Opção inválida: {other:?}"),
            }
        }
    }

    fn ask_label(
        &mut self,
        id: ProfileId,
        current: Option<String>,
        validate: impl Fn(&str) -> Result<(), String>,
    ) -> Result<String> {
        let current = current.filter(|label| validate(label).is_ok());

        loop {
            println!();
            println!("Digite um nome para o perfil {id}:");
            if let Some(label) = &current {
                println!("Tecle ENTER para manter o nome \"{label}\"");
            }

            let label = self.readline()?;
            if label.is_empty() {
                if let Some(label) = current {
                    return Ok(label);
                }
                print_error!("O nome não pode ser vazio");
            } else if let Err(msg) = validate(&label) {
                print_error!("{msg}");
            } else {
                return Ok(label);
            }
        }
    }

    fn readline(&mut self) -> Result<String> {
        let input = self.inner.readline.readline("> ")?;
        Ok(input)
    }

    fn configs(&self) -> &ConfigsWriter {
        &self.inner.configs
    }

    fn configs_mut(&mut self) -> &mut ConfigsWriter {
        &mut self.inner.configs
    }

    fn finalize(self) -> Result<()> {
        self.restore_initial_profile()
    }

    fn restore_initial_profile(&self) -> Result<()> {
        let current_profile = get_profile()?;
        if current_profile != self.initial_profile {
            set_profile(&self.initial_profile, SetProfileAction::Apply)?;
        }
        Ok(())
    }
}

#[expect(clippy::unnecessary_wraps)]
fn accept_anything<T: ?Sized>(_: &T) -> Result<(), String> {
    Ok(())
}
