let dbg = Printf.printf "[debug]%s"

let id = fun x -> x

let scanf fmt f = Scanf.sscanf (read_line ()) fmt f

let scan_matrix n m e conv =
  let arr = Array.make_matrix n m e in
  Array.iteri (fun i line ->
      let s = Scanf.Scanning.from_string @@ read_line () in
      Array.iteri (fun j _ ->
          arr.(i).(j) <- Scanf.bscanf s " %s" conv;
        ) line) arr;
  arr

let (++) n m =
  let rec aux i =
    if i = m then [m]
    else i :: aux (i+1) in
  if n > m then [] else aux n

let (++^) n m = n ++ (m-1)

let between n x m = n <= x && x < m

let n = scanf "%d" (fun d -> d)
let arr = Array.init n (fun _-> [])
let found = Array.init n (fun _ -> Queue.create ())

let rec dfs visited i n =
  Queue.push n found.(i);
  let m = List.fold_left (fun m j ->
      if not visited.(j) then
        (visited.(j) <- true;
         dfs visited j m)
      else m) (n+1) arr.(i) in
  Queue.push m found.(i);
  m+1

let ()=
  List.iter (fun _ ->
      let b = Scanf.Scanning.from_string @@ read_line () in
      let u = Scanf.bscanf b "%d" id in
      let m = Scanf.bscanf b " %d" id in
      List.iter (fun _ ->
          arr.(u-1) <- arr.(u-1) @ [Scanf.bscanf b " %d" id - 1])
        (0++^m)) (0++^n);
  let visited = Array.make n false in
  let n = ref 1 in
  Array.iteri (fun i v ->
      if not v then
        (visited.(i) <- true;
         n := dfs visited i !n)) visited;
  Array.iteri (fun i q ->
      Printf.printf "%d %s\n" (i+1) @@
      String.concat " " @@ List.map string_of_int @@ List.rev
      @@ Queue.fold (fun l h -> h::l) [] q) found
