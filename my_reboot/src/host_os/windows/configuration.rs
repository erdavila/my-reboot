use ansi_term::Color;
use anyhow::Result;
use display_profile_lib::{Profile, Rotation, SetProfileAction, get_profile, set_profile};
use rustyline::DefaultEditor;

use crate::options_types::{LabeledProfile, ProfileId};
use crate::persist::configs::ConfigsWriter;

pub(crate) fn configure() -> Result<()> {
    let mut configure = Configure::new()?;
    configure.configure()?;
    configure.finalize()?;
    Ok(())
}

macro_rules! print_error {
    ($($tt:tt)*) => {
        println!("{} {}", Color::Red.paint("Erro:"), format_args!($($tt)*))
    };
}

struct Configure {
    initial_profile: Profile,
    readline: DefaultEditor,
}

impl Configure {
    fn new() -> Result<Self> {
        let initial_profile = get_profile()?;
        let readline = DefaultEditor::new()?;
        Ok(Self {
            initial_profile,
            readline,
        })
    }

    #[expect(clippy::similar_names)]
    fn configure(&mut self) -> Result<()> {
        println!(
            "Configuraremos perfis {} e {} de telas do Windows.",
            ProfileId::A,
            ProfileId::B
        );

        let profile_a = self.configure_profile(ProfileId::A, accept_anything)?;
        let profile_a_label = self.ask_label(ProfileId::A, accept_anything)?;

        let profile_b = self.configure_profile(ProfileId::B, |profile| {
            if *profile == profile_a {
                Err(format!(
                    "A configuração não pode ser igual à do perfil {}",
                    ProfileId::A
                ))
            } else {
                Ok(())
            }
        })?;
        let profile_b_label = self.ask_label(ProfileId::B, |label| {
            if *label == profile_a_label {
                Err(format!(
                    "O nome não pode ser igual ao do perfil {}",
                    ProfileId::A
                ))
            } else {
                Ok(())
            }
        })?;

        println!();

        println!("Resumo dos perfis:");
        Self::print_profile_summary(ProfileId::A, &profile_a_label, &profile_a);
        Self::print_profile_summary(ProfileId::B, &profile_b_label, &profile_b);
        println!();

        let mut configs = ConfigsWriter::load()?;
        configs.set_profile(ProfileId::A, &profile_a_label, &profile_a)?;
        configs.set_profile(ProfileId::B, &profile_b_label, &profile_b)?;
        println!("Salvando as configurações...");
        configs.save()?;
        println!();

        println!("Configuração finalizada.");
        Ok(())
    }

    fn configure_profile(
        &mut self,
        id: ProfileId,
        validate: impl Fn(&Profile) -> Result<(), String>,
    ) -> Result<Profile> {
        loop {
            println!();
            println!("Escolha uma das opções para configurar o perfil {id}:");
            println!("1. A configuração de tela atual corresponde ao perfil {id}");
            println!("2. Abrir as configurações de tela do Windows");

            match self.readline.readline("> ")?.as_str() {
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
                other => print_error!("Opção inválida: {other:?}"),
            }
        }
    }

    fn ask_label(
        &mut self,
        id: ProfileId,
        validate: impl Fn(&str) -> Result<(), String>,
    ) -> Result<String> {
        loop {
            println!();
            let label = self
                .readline
                .readline(&format!("Digite um nome para o perfil {id}: "))?;
            if label.is_empty() {
                print_error!("O nome não pode ser vazio");
            } else if let Err(msg) = validate(&label) {
                print_error!("{msg}");
            } else {
                return Ok(label);
            }
        }
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

    fn print_profile_summary(id: ProfileId, label: &str, profile: &Profile) {
        println!("  {}", LabeledProfile::new(id, label));
        for monitor in profile {
            print!(
                "    {}: {}x{}; {:.2}Hz; em {},{}",
                monitor.friendly_device_name,
                monitor.dimensions.width,
                monitor.dimensions.height,
                f64::from(monitor.refresh_rate.numerator)
                    / f64::from(monitor.refresh_rate.denominator),
                monitor.position.x,
                monitor.position.y,
            );

            let rotation = match monitor.rotation {
                Rotation::IDENTITY => None,
                Rotation::ROTATE90 => Some(90),
                Rotation::ROTATE180 => Some(180),
                Rotation::ROTATE270 => Some(270),
            };
            if let Some(rotation) = rotation {
                print!("; rotação de {rotation}°");
            }

            println!();
        }
    }
}

#[expect(clippy::unnecessary_wraps)]
fn accept_anything<T: ?Sized>(_: &T) -> Result<(), String> {
    Ok(())
}
