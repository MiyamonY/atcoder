module ArrayL = ArrayLabels
module ListL = ListLabels

let dbg = Printf.printf "[debug]%s"

let max_num = 100_000_000_000

let id = fun x -> x
let tuple2 x y = (x,y)
let tuple3 x y z = (x,y,z)
let succ x = x + 1
let pred x = x - 1

let (++) n m =
  let rec aux acc i =
    if i = m then m::acc
    else aux (i::acc) (i+1) in
  if n > m then [] else List.rev @@ aux [] n

let (++^) n m = n ++ (m-1)

let scan fmt f = Scanf.sscanf (read_line ()) fmt f

let scan_list ~sep cnv =
  let line = read_line () in
  Str.split (Str.regexp_string sep) line
  |> List.map cnv

let scan_lines n fmt f =
  List.map (fun _ -> scan fmt f) (0++^n)

let scan_matrix n m e conv =
  let arr = Array.make_matrix n m e in
  Array.iteri (fun i line ->
      let s = Scanf.Scanning.from_string @@ read_line () in
      Array.iteri (fun j _ ->
          arr.(i).(j) <- Scanf.bscanf s " %s" conv;
        ) line) arr; arr

let between n x m = n <= x && x < m

let string_to_list s =
  List.map (String.get s) (0 ++^ String.length s)

let rec powerset = function
  | [] -> [[]]
  | hd::tl ->
    let pws =  powerset tl in
    pws @ ListL.map pws ~f:(fun pw -> hd::pw)

let permutations l =
  let rec interleave x = function
    | [] -> [[x]]
    | (hd::tl) as l ->
      (x::l) :: (interleave x tl |> ListL.map ~f:(fun l -> hd::l)) in
  let rec aux = function
    | [] -> [[]]
    | a::rest ->
      aux rest |> ListL.map ~f:(interleave a) |> List.concat in
  aux l

let n = scan "%d" id
let arr = Array.of_list @@ scan_list ~sep:" " int_of_string
let q = scan "%d" id
let ls = scan_list ~sep:" " int_of_string

exception Found

let () =
  let pwss = powerset (0 ++^ n) in
  let ht = Hashtbl.create @@ List.length pwss in
  ListL.iter pwss ~f:(fun pws ->
      ListL.map pws ~f:(fun i -> arr.(i))
      |> ListL.fold_left ~init:0 ~f:(+)
      |> (fun v-> Hashtbl.add ht v true));
  ListL.iter ls ~f:(fun v ->
      if Hashtbl.mem ht v then Printf.printf "yes\n"
      else  Printf.printf "no\n")
