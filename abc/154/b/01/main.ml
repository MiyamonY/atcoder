open Printf

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

let id a = a

let s = scan "%s" id

let () =
  String.iter (fun c -> printf "x") s;
  printf "\n"
