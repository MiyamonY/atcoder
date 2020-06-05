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

let n = scan "%d" id
let ns = scan_lines n "%d %d" tuple2

let m = scan "%d" id
let ms = scan_lines m "%d %d" tuple2

exception Found of int * int
let () =
  try ListL.iter ms ~f:(fun (x,y) ->
      let (x0,y0) = List.hd ns in
      let (dx, dy) = x-x0, y-y0 in
      if List.for_all (fun v -> List.exists ((=) v) ms)
          (ListL.map ns ~f:(fun (x,y) -> x+dx, y+dy))
      then raise (Found (dx,dy))) with
  | Found (dx,dy) -> Printf.printf "%d %d\n" dx dy
