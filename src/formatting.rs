use book::{Book, Metadata, MetadataField};

pub static DEFAULT_FMT: FmtStr = FmtStr("%title% - %creator%");
static FMTERS: [(&'static str, MetadataField); 15] = [
	("title", MetadataField::Title),
	("language", MetadataField::Language),
	("identifier", MetadataField::Identifier),
	("creator", MetadataField::Creator),
	("contributor", MetadataField::Contributor),
	("publisher", MetadataField::Publisher),
	("subject", MetadataField::Subject),
	("description", MetadataField::Description),
	("date", MetadataField::Date),
	("type_tag", MetadataField::TypeTag),
	("format", MetadataField::Format),
	("source", MetadataField::Source),
	("relation", MetadataField::Relation),
	("coverage", MetadataField::Coverage),
	("rights", MetadataField::Rights),
];

fn find_fmter(s: &str, fmters: &[(&str, MetadataField)]) -> Option<MetadataField> {
	for &(fmt, f) in fmters {
		if s == fmt { return Some(f) };
	}

	None
}

#[derive(Debug)]
pub struct Error {
	pub idx: usize,
	pub pat: String,
}

pub struct FmtStr<'a>(&'a str);

impl<'a> FmtStr<'a> {
	pub fn format_book(&self, book: &Book) -> Result<String, Error> {
		let md = &book.meta;
		let &FmtStr(fmt) = self;

		let mut out = String::new();
		let mut last_i = 0;
		let mut inside = false;
		let mut it = fmt.match_indices('%');
		while let Some((i, _)) = it.next() {
			if i < fmt.len() - 1 && &fmt[i + 1..i + 2] == "%" && !inside {
				debug!("Escape %");
				out.push_str("%");
				it.next();
			} else if inside {
				debug!("Replace %");
				let pat = &fmt[last_i..i];
				let fmter = match find_fmter(pat, &FMTERS) {
					None => return Err(Error { idx: i, pat: pat.to_string() }),
					Some(f) => f,
				};

				let rep = fmter.lookup(md).unwrap();
				out.push_str(rep);
				inside = false;
			} else {
				debug!("Push middle");
				out.push_str(&fmt[last_i..i]);
				inside = true;
			};
			last_i = i + 1;
		}

		if inside {
			return Err(Error { idx: last_i, pat: "%".to_string() });
		};

		out.push_str(&fmt[last_i..fmt.len()]);

		Ok(out)
	}
}
