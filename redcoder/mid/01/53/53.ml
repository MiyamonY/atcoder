let dbg = Printf.printf "[debug]%s"

let max_num = 1_000_000_000

let id = fun x -> x
let tuple2 x y = (x,y)
let succ x = x + 1
let pred x = x - 1

let (++) n m =
  let rec aux i =
    if i = m then [m]
    else i :: aux (i+1) in
  if n > m then [] else aux n

let (++^) n m = n ++ (m-1)

let scan fmt f = Scanf.sscanf (read_line ()) fmt f

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

let lower_bound n f =
  let rec aux l u =
    if u - l > 1 then
      let mid = (l + u) /2 in
      if f mid then
        aux l mid
      else aux mid u
    else u in aux (-1) n


let n = scan "%d" id
let ls = scan_lines n "%d" id

let arr = Array.make n 0

let rec solve len = function
  | [] -> len
  | v::rest ->
    if len = 0 || arr.(pred len) < v then
      (arr.(len) <- v;
       solve (succ len) rest)
    else
      let i = lower_bound len (fun i -> v <= arr.(i)) in
      arr.(i) <- v;
      solve len rest

let () =

  Printf.printf "%d\n" @@ solve 0 ls
