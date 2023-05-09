use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Gie {
    #[clap(help = "Get inscription entry of <INSCRIPTION_ID>.")]
    inscription_id: InscriptionId,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub fee: u64,
  pub height: u64,
  pub number: u64,
  pub sat: Option<Sat>,
  pub timestamp: u32,
  pub media_type: String,
  pub media_size: usize,
  pub media_content: String,
}


impl Gie {
    pub(crate) fn run(self, options: Options) -> Result {
        let index = Index::open(&options)?;
        let inscription_id = self.inscription_id;

        let inscription = index
            .get_inscription_by_id(inscription_id)?;

        let mut media_type: String = String::new();
        let mut media_content: String = String::new();
        let mut media_size: usize = 0;

        match inscription {
            Some(inscription) => {
                media_type = inscription
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .parse()
                    .unwrap();

                match inscription.content_length() {
                    Some(size) => media_size = size,
                    None => {}
                };

                match inscription.into_body() {
                    Some(body) => media_content = base64::encode(body),
                    None => {}
                };
            },
            None => {}
        };

        match index.get_inscription_entry(inscription_id)? {
            Some(entry) => {
                print_json(Output { 
                    fee: entry.fee,
                    height: entry.height,
                    number: entry.number,
                    sat: entry.sat,
                    timestamp: entry.timestamp,
                    media_type,
                    media_size,
                    media_content
                })?;
                Ok(())
            }
            None => Err(anyhow!("no inscription found..")),
        }
    }
}
