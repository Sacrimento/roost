use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::{i32, i64, u8, vec};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    root_file: PathBuf,
}

#[derive(Debug)]
enum Entry {
    Integer32(i32),
    Integer64(i64),
    Text(String),
}
impl Entry {
    fn convert_chunk(&self, chunk: Vec<u8>) -> Self {
        match self {
            Self::Integer64(_) => Self::Integer64(<i64>::from_be_bytes(
                chunk.try_into().expect("ouai ouai ouai ouai"),
            )),
            Self::Integer32(_) => Self::Integer32(<i32>::from_be_bytes(
                chunk.try_into().expect("ouai ouai ouai ouai"),
            )),
            Self::Text(_) => Self::Text(chunk.iter().map(|i| *i as char).collect::<String>()),
        }
    }
}

fn main() {
    let args = Args::parse();

    let reader = BufReader::new(File::open(&args.root_file).expect("Unable to open root_file"));

    // let header_format = String::from(">4sii iiiiiBiiiH16s");

    let header_entries = [
        (Entry::Text(String::new()), 4),
        (Entry::Integer32(0), 4),
        (Entry::Integer32(0), 4),
        (Entry::Integer32(0), 4),
        (Entry::Integer32(0), 4),
        (Entry::Integer32(0), 4),
        (Entry::Integer32(0), 4),
        (Entry::Integer32(0), 4),
        (Entry::Integer32(0), 1),
        (Entry::Integer32(0), 4),
        (Entry::Integer32(0), 4),
        (Entry::Integer32(0), 4),
    ];

    let mut bytes = reader.bytes();

    for header_entry in header_entries {
        let mut byte_chunk: Vec<u8> = vec![];

        for _ in 0..header_entry.1 {
            if let Some(byte) = bytes.next() {
                byte_chunk.push(byte.unwrap());
            }
        }
        let test = header_entry.0.convert_chunk(byte_chunk);

        println!("{:?}", test);
    }
}

//         self._file_path = file._file_path
//         self._options = file._options
//         self._fVersion = file._fVersion
//         self._fBEGIN = file._fBEGIN
//         self._fEND = file._fEND
//         self._fSeekFree = file._fSeekFree
//         self._fNbytesFree = file._fNbytesFree
//         self._nfree = file._nfree
//         self._fNbytesName = file._fNbytesName
//         self._fUnits = file._fUnits
//         self._fCompress = file._fCompress
//         self._fSeekInfo = file._fSeekInfo
//         self._fNbytesInfo = file._fNbytesInfo
//         self._fUUID_version = file._fUUID_version
//         self._fUUID = file._fUUID

// _file_header_fields_small = struct.Struct(">4siiiiiiiBiiiH16s") 32 bits ?
// _file_header_fields_big = struct.Struct(">4siiqqiiiBiqiH16s") 64 bits ?

// begin chunk size = 403   (the smallest a root file can be)

// from jsroot

// async readKeys() {
//       // with the first readbuffer we read bigger amount to create header cache
//       return this.readBuffer([0, 400]).then(blob => {
//          const buf = new TBuffer(blob, 0, this);
//          if (buf.substring(0, 4) !== 'root')
//             return Promise.reject(Error(`Not a ROOT file ${this.fURL}`));

//          buf.shift(4);

//          this.fVersion = buf.ntou4();
//          this.fBEGIN = buf.ntou4();
//          if (this.fVersion < 1000000) { // small file
//             this.fEND = buf.ntou4();
//             this.fSeekFree = buf.ntou4();
//             this.fNbytesFree = buf.ntou4();
//             buf.shift(4); // const nfree = buf.ntoi4();
//             this.fNbytesName = buf.ntou4();
//             this.fUnits = buf.ntou1();
//             this.fCompress = buf.ntou4();
//             this.fSeekInfo = buf.ntou4();
//             this.fNbytesInfo = buf.ntou4();
//          } else { // new format to support large files
//             this.fEND = buf.ntou8();
//             this.fSeekFree = buf.ntou8();
//             this.fNbytesFree = buf.ntou4();
//             buf.shift(4); // const nfree = buf.ntou4();
//             this.fNbytesName = buf.ntou4();
//             this.fUnits = buf.ntou1();
//             this.fCompress = buf.ntou4();
//             this.fSeekInfo = buf.ntou8();
//             this.fNbytesInfo = buf.ntou4();
//          }

//          // empty file
//          if (!this.fSeekInfo || !this.fNbytesInfo)
//             return Promise.reject(Error(`File ${this.fURL} does not provide streamer infos`));

//          // extra check to prevent reading of corrupted data
//          if (!this.fNbytesName || this.fNbytesName > 100000)
//             return Promise.reject(Error(`Cannot read directory info of the file ${this.fURL}`));

//          // *-*-------------Read directory info
//          let nbytes = this.fNbytesName + 22;
//          nbytes += 4;  // fDatimeC.Sizeof();
//          nbytes += 4;  // fDatimeM.Sizeof();
//          nbytes += 18; // fUUID.Sizeof();
//          // assume that the file may be above 2 Gbytes if file version is > 4
//          if (this.fVersion >= 40000)
