use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    root_file: PathBuf,
}

// #[derive(Debug)]
// enum Entry {
//     Integer32(i32),
//     Integer64(i64),
//     Text(String),
// }
// impl Entry {
//     fn convert_chunk(&self, chunk: Vec<u8>) -> Self {
//         match self {
//             Self::Integer64(_) => Self::Integer64(<i64>::from_be_bytes(
//                 chunk.try_into().expect("ouai ouai ouai ouai"),
//             )),
//             Self::Integer32(_) => Self::Integer32(<i32>::from_be_bytes(
//                 chunk.try_into().expect("ouai ouai ouai ouai"),
//             )),
//             Self::Text(_) => Self::Text(chunk.iter().map(|i| *i as char).collect::<String>()),
//         }
//     }
// }

//TFileHeader ?
#[derive(Default, Debug)]
struct FileHeader {
    version: i32,
    begin: i32,
    end: i32,
    seekfree: i64,
    nbytesfree: i32,
    // nfree this is 4 bytes and should be skippd apparently ?
    nbytesname: i32,
    units: i8,
    compress: i32,
    seekinfo: i64,
    nbytesinfo: i32,
}

// impl Default for FileHeader {
//     fn default() -> Self {
//         Self {
//             version: 0,
//             begin: 0,
//             end: 0,
//             seekfree: 0,
//             nbytesfree: 0,
//             // nfree this is 4 bytes and should be skippd apparently ?
//             nbytesname: 0,
//             units: 0,
//             compress: 0,
//             seekinfo: 0,
//             nbytesinfo: 0,
//         }
//     }
// }

// Should it be named TFile instead ?
struct RootFile {
    header: FileHeader,
}

fn main() {
    let args = Args::parse();

    let mut reader = BufReader::new(File::open(&args.root_file).expect("Unable to open root_file"));

    // let header_format = String::from(">4s i i i i i i i B i i i H 16s");

    let mut file = FileHeader::default();

    let mut buf = [0; 4];
    reader.read(&mut buf[..]).unwrap();
    // println!("{}", );
    // file.magic = std::str::from_utf8(&buf).unwrap();

    reader.read(&mut buf[..]).unwrap();

    file.version = i32::from_be_bytes(buf);

    let is_big = file.version > 1_000_000;
    let mut big_buf: Vec<u8> = Vec::new();
    big_buf.resize(if is_big { 67 } else { 55 }, 0);
    let cursor_shift = if is_big { 2 } else { 1 };

    reader.read(&mut big_buf[..]).unwrap();

    println!("{:?}", big_buf);

    let mut cursor = 4;
    file.begin = i32::from_be_bytes(big_buf[0..cursor].try_into().unwrap());


    file.end = i32::from_be_bytes(
        big_buf[cursor..(cursor + 4 * cursor_shift)]
            .try_into()
            .unwrap(),
    );
    cursor += 4 * cursor_shift;
    file.seekfree = i64::from_be_bytes(
        big_buf[cursor..(cursor + 4 * cursor_shift)]
            .try_into()
            .unwrap(),
    );
    cursor += 4 * cursor_shift;
    file.nbytesfree = i32::from_be_bytes(big_buf[cursor..cursor + 4].try_into().unwrap());
    cursor += 8; // skipping nfree 
    file.nbytesname = i32::from_be_bytes(big_buf[cursor..cursor + 4].try_into().unwrap()); //i32 ????? name ???
    cursor += 4;
    file.units = i8::from_be_bytes(big_buf[cursor..cursor + 1].try_into().unwrap());
    cursor += 1;
    file.compress = i32::from_be_bytes(big_buf[cursor..cursor + 4].try_into().unwrap());
    cursor += 4;
    file.seekinfo = i64::from_be_bytes(
        big_buf[cursor..(cursor + 4 * cursor_shift)]
            .try_into()
            .unwrap(),
    );
    cursor += 4 * cursor_shift;

    file.nbytesinfo = i32::from_be_bytes(big_buf[cursor..cursor + 4].try_into().unwrap());
    // cursor += 4;

    println!("{:?}", file);

    // seekfree: i64,
    // nbytesfree: i32,
    // // nfree this is 4 bytes and should be skippd apparently ?
    // nbytesname: i32,
    // units: i8,
    // compress: i32,
    // seekinfo: i64,
    // nbytesinfo: i32,
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
// _file_header_fields_big = struct.Struct(">4s i i q q i i i B i q i H 1 6 s") 64 bits ?

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

// if self._fSeekKeys == 0:
//             self._header_key = None
//             self._keys = []
//             self._keys_lookup = {}
//             self._len = None
//         else:
//             keys_start = self._fSeekKeys
//             keys_stop = min(keys_start + self._fNbytesKeys + 8, file.fEND)
//             keys_cursor = uproot.source.cursor.Cursor(self._fSeekKeys)

//             self.hook_before_read_keys(
//                 chunk=chunk, cursor=cursor, keys_cursor=keys_cursor
//             )

//             if (keys_start, keys_stop) in chunk:
//                 keys_chunk = chunk
//             else:
//                 # Chunk will not be retained; we don't have to detach_memmap()
//                 keys_chunk = file.chunk(keys_start, keys_stop)

//             self.hook_before_header_key(
//                 chunk=chunk,
//                 cursor=cursor,
//                 keys_chunk=keys_chunk,
//                 keys_cursor=keys_cursor,
//             )

//             # header_key is never used, but we do need to seek past it
//             ReadOnlyKey(keys_chunk, keys_cursor, {}, file, self, read_strings=True)

//             num_keys = keys_cursor.field(
//                 keys_chunk, _directory_format_num_keys, context
//             )

//             self.hook_before_keys(
//                 chunk=chunk,
//                 cursor=cursor,
//                 keys_chunk=keys_chunk,
//                 keys_cursor=keys_cursor,
//                 num_keys=num_keys,
//             )

//             self._keys = []
//             self._keys_lookup = {}
//             for _ in range(num_keys):
//                 key = ReadOnlyKey(
//                     keys_chunk, keys_cursor, {}, file, self, read_strings=True
//                 )
//                 name = key.fName
//                 if name not in self._keys_lookup:
//                     self._keys_lookup[name] = []
//                 self._keys_lookup[name].append(len(self._keys))
//                 self._keys.append(key)
