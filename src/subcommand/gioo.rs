use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Gioo {
    #[clap(help = "Get inscriptions of <OUTPOINT>.")]
    outpoint: OutPoint,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub inscriptions: Vec<InscriptionId>,
}


impl Gioo {
    pub(crate) fn run(self, options: Options) -> Result {
        let index = Index::open(&options)?;

        let inscriptions = index.get_inscriptions_on_output(self.outpoint)?;

        print_json(Output { inscriptions })?;

        Ok(())
    }
}
