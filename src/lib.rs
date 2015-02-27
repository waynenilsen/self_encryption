/*  Copyright 2014 MaidSafe.net limited

    This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
    version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
    licence you accepted on initial access to the Software (the "Licences").

    By contributing code to the MaidSafe Software, or to this project generally, you agree to be
    bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
    directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
    available at: http://www.maidsafe.net/licenses

    Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
    under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
    OF ANY KIND, either express or implied.

    See the Licences for the specific language governing permissions and limitations relating to
    use of the MaidSafe
    Software.                                                                 */

//! A file **content** self encryptor
//! 
//! This library will provide convergent encryption on file based data and produce a 
//! ```DataMap``` type and several chunks of data. Each chunk is max 1Mb in size
//! and has a name. Thsi name is the ``Sah512``` of the content. This allows the chunks
//! to be confirmed and if using size and Hash checks then there is a high degree of certainty
//! in the data validity. 
//! 
//! # Use
//!
//!
//! ### Examples
//!
//! ```rust
//! extern crate self_encryption;
//!
//! # fn main() {}
//! ```
//!

#![doc(html_logo_url = "http://maidsafe.net/img/Resources/branding/maidsafe_logo.fab2.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
       html_root_url = "http://doc.rust-lang.org/log/")]
#![warn(missing_docs)]


extern crate rand;
extern crate crypto;
use std::collections::HashMap;
use std::cmp;
use std::old_io::TempDir;
// this is pub to test the tests dir integration tests these are temp and need to be
// replaced with actual integration tests and this should be private
mod encryption;
/// Holds pre and post encryption hashes as well as original chunk size
pub mod datamap;

static MaxChunkSize: u32 = 1024*1024;
static MinChunkSize: u32 = 1024;
  /// Will use a tempdir to stream un procesed data, although this is done vie AES streaming with 
  /// a randome key and IV
  pub fn create_temp_dir() ->TempDir {
    match TempDir::new("self_encryptor") {
      Ok(dir) => dir,
        Err(e) => panic!("couldn't create temporary directory: {}", e)
    }
  }

/// This is the encryption object and all file handling should be done via this as the low level 
/// mechanism to read and write *content* this librar has no knowledge of file metadata. This is
/// a library to ensure content is secured 
pub struct SelfEncryptor {
  /* this_data_map: DataMap, */
  /* sequencer: Vec<u8>, */
  /* chunks: HashMap::new(), */
  tempdir : TempDir, 
  file_size: u64,
  closed: bool,
  }


impl SelfEncryptor {
  /// constructor for encryptor object
  pub fn new(tempdir: TempDir, file_size: u64, closed: bool)-> SelfEncryptor {
    SelfEncryptor{tempdir: tempdir, file_size: file_size, closed: closed}
    }
  /// Write method mirrors a posix type write mechanism
  pub fn write(&mut self, data: &str ,length: u32, position: u64) {
    let new_size = cmp::max(self.file_size, length as u64 + position);
    /* self.Preparewindow(length, position, true); */
    /* for i in 0u64..length as u64 { */
    /*   self.sequencer[position + i] = data[i] as u8; */
    /*   } */
    /*   */
    self.file_size = new_size;
  }
  /// current file size as is known by encryptor
  pub fn len(&self)->u64 {
    self.file_size
  } 
  /// Prepere a sliding window to ensure there are enouch chunk slots for write
  /// will possibly readin some chunks from external storage
  fn prepare_window(&mut self, length: u32, position: u64, write: bool) {
  }
  // Helper methods
  fn get_num_chunks(&self)->u32 {
    if self.file_size  < (3 * MinChunkSize as u64) { return 0 }
    if self.file_size  < (3 * MaxChunkSize as u64) { return 3 }
    if self.file_size  % MaxChunkSize as u64 == 0 {
      return (self.file_size / MaxChunkSize as u64) as u32 
      } else {
      return (self.file_size / MaxChunkSize as u64 + 1) as u32
        }
    }

  fn get_chunk_size(&self, chunk: u32)->u32 {
    if self.file_size < 3 * MinChunkSize as u64 { return 0u32 }
    if self.file_size < 3 * MaxChunkSize as u64 { 
      if chunk < 2 { 
        return (self.file_size / 3) as u32 
      } else {
        return (self.file_size - (2 * self.file_size / 3)) as u32 
      }
    }
    if chunk < SelfEncryptor::get_num_chunks(self) - 2 { return MaxChunkSize }
    let remainder :u32 = self.file_size as u32 % MaxChunkSize;
    let penultimate :bool = (SelfEncryptor::get_num_chunks(self) - 2) == chunk;
    if remainder == 0 { return MaxChunkSize }
    if remainder < MinChunkSize {
       if penultimate { return MaxChunkSize - MinChunkSize 
         } else { 
           return MinChunkSize + remainder } 
      } else {
        if penultimate { return MaxChunkSize } else { return remainder }
        }
    
  }

  fn get_start_end_position(&self, chunk :u32)->(u64, u64) {
   if self.get_num_chunks() == 0 { return (0,0) } 
   let mut start :u64;
   let penultimate = (self.get_num_chunks() - 2) == chunk;
   let last = (self.get_chunk_size(0) - 1) == chunk; 
   if last {
     start = (self.get_chunk_size(0) * (chunk - 2) + self.get_chunk_size(chunk - 2) +
       self.get_chunk_size(chunk - 1)) as u64;
   } else if penultimate {
     start = (self.get_chunk_size(0) * (chunk - 1) + self.get_chunk_size(chunk - 1)) as u64;
   } else {
     start = (self.get_chunk_size(0) * chunk) as u64;
   }
    (start, (start + self.get_chunk_size(chunk) as u64))
    }
}


#[test]
fn check_write() {
  let mut se = SelfEncryptor::new(create_temp_dir(),  0, false);
  let mut se_ctr = SelfEncryptor{tempdir: create_temp_dir(), file_size: 0, closed: false};
  se.write("dsd", 3u32, 5u64);
  se_ctr.write("fkghguguykghj", 30u32, 50u64);
  assert_eq!(se.file_size, 8u64);
  assert_eq!(se_ctr.file_size, 80u64);
}
