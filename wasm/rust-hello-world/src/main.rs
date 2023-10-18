fn main() {

    // use of strings literal for multi-line string
    // https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
    let hello = r#"
!... Hello Podman wasm World ...!

         .--"--.
       / -     - \
      / (O)   (O) \
   ~~~| -=(,Y,)=- |
    .---. /`  \   |~~
 ~/  o  o \~~~~.----. ~~
  | =(X)= |~  / (O (O) \
   ~~~~~~~  ~| =(Y_)=-  |
  ~~~~    ~~~|   U      |~~

Project:   https://github.com/containers/podman
Website:   https://podman.io
Documents: https://docs.podman.io
Twitter:   @Podman_io
"#;
    println!("{}", hello);
    
  }
  