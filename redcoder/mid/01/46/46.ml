let dbg = Printf.printf "[debug]%s"

let max_num = 1_000_000_000

module Tuple2 = struct
  let make x y = (x, y)
  let first (x, _) = x
  let second (_, y) = y
end

let id = fun x -> x
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
let arr = Array.of_list @@ scan_lines n "%d %d" Tuple2.make

let memo = Array.make_matrix n n None

let rec solve n m =
  match memo.(n).(m) with
  | Some v ->  v
  | None ->
    if n = m then 0
    else
      let v = List.map (fun i ->
          let v = Tuple2.first arr.(n) * Tuple2.second arr.(i) * Tuple2.second arr.(m) in
          (solve n i) + (solve (i+1) m) + v) (n ++^ m)
              |> List.fold_left min max_num in
      memo.(n).(m) <- Some v;
      v

let () =
  solve 0 @@ pred n
  |> Printf.printf "%d\n"
