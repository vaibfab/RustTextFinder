///////////////////////////////////////////////////////////
// BuildOnStructure::TextSearch.rs                       //
//   - demonstrate code structure for Step #1            //
// Jim Fawcett, https://JimFawcett.github.io, 14 Jan 21  //
///////////////////////////////////////////////////////////

use std::path::{Path, PathBuf};

/*-----------------------------------------------
  A trait is similar to an interface, most often
  used to convey a communication protocol for
  generic types.

  SearchEvent trait is not needed here, but will be in 
  Step #4
*/
pub trait SearchEvent {
    fn set_dir(&mut self, dir: &Path);
    fn set_file(&mut self, rslt: (&Path, bool, &str));
}
/*-----------------------------------------------
  GenOut converts Finder's data into information
  for user.
*/
#[derive(Debug)]
pub struct GenOut {
    dir: PathBuf
}
/*-- implement trait --------------------------*/
impl SearchEvent for GenOut {
    fn set_dir(&mut self, rdir: &Path) {
        self.dir = rdir.to_path_buf();
        print!("\n  {:?}", rdir);
    }
    fn set_file(&mut self, rslt: (&Path, bool, &str)) {
        let (file, found, text) = rslt;
        if found {
            print!("\n    {:?}: {:?} found", file, text);
        }
        else {
            print!("\n    {:?}: {:?} not found", file, text);
        }
    }
}
/*-- implement other member functions ---------*/
impl GenOut {
    pub fn new() -> GenOut {
        GenOut {
            dir: PathBuf::new()
        }
    }
}
/*-----------------------------------------------
  Trait DirEvents specifies functions that
  Finder provides to support DirNav calls.

  Not used in this step.
*/
pub trait DirEvent {
    fn do_dir(&mut self, dir: &Path);
    fn do_file(&mut self, file: &Path);
}
/*-----------------------------------------------
  Finder searches for text in specified file
*/
#[derive(Debug)]
pub struct Finder {
      dir: PathBuf,
      srctxt: String,
      out: GenOut
}
/*-- implement DirEvent --*/
impl DirEvent for Finder {
    fn do_dir(&mut self, dir: &Path) {
        self.dir = dir.to_path_buf();
        self.out.set_dir(dir);
    }
    fn do_file(&mut self, file: &Path) {
        /* 
            Pretending to search for text in file.
            Function should:
              1. append flnm to dir
              2. attempt to open file
              3. search for text
              4. send result to out 
        */
        if self.srctxt == "BuildOn" {
            self.out.set_file((file, true, &self.srctxt));
        }
        else {
            self.out.set_file((file, false, &self.srctxt));
        }
    }
}
/*-- implement Finder methods -----------------*/
impl Finder {
    pub fn new() -> Finder {
        Finder {
            dir: PathBuf::new(),
            srctxt: String::new(),
            out: GenOut::new()
        }
    }
    pub fn set_txt(&mut self, txt: &str) {
        self.srctxt = txt.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct_genout() {
        let g = GenOut::new();
        assert_eq!(g.dir , PathBuf::from(""));
    }
    #[test]
    fn construct_finder() {
        let f = Finder::new();
        assert_eq!(f.dir,  PathBuf::from(""));
    }
}