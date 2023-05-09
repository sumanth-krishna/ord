use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Giibsp {
  #[clap(help = "Get inscription id by <SATPOINT>.")]
  satpoint: SatPoint,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub inscription_id: InscriptionId,
}

impl Giibsp {
  pub(crate) fn run(self, options: Options) -> Result {
    let index = Index::open(&options)?;

    match index.get_inscription_id_by_satpoint(self.satpoint)? {
      Some(inscription_id) => {
        print_json(Output { inscription_id })?;
        Ok(())
      }
      None => Err(anyhow!("no inscription found..")),
    }
  }
}
