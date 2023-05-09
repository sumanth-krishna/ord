use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Giibs {
  #[clap(help = "Get inscription id of <SAT>.")]
  sat: Sat,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub inscription_id: InscriptionId,
}

impl Giibs {
  pub(crate) fn run(self, options: Options) -> Result {
    let index = Index::open(&options)?;

    match index.get_inscription_id_by_sat(Sat(self.sat.0))? {
      Some(inscription_id) => {
        print_json(Output { inscription_id })?;
        Ok(())
      }
      None => Err(anyhow!("no inscription found..")),
    }
  }
}
