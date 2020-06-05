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

module Heap = Set.Make(struct
    type t = int * int
    let compare = compare
  end)

let (v, e) = scan "%d %d" tuple2
let edges =
  let ed = Array.make v [] in
  let l = scan_lines e "%d %d %d" tuple3 in
  ListL.iter l ~f:(fun (s,t,d) ->
      ed.(s) <- (t,d) :: ed.(s);
      ed.(t) <- (s,d) :: ed.(t);); ed

let visited = Array.make v false

let prim =
  let rec aux n h =
    if Heap.is_empty h then n
    else
      let (d, s) = Heap.min_elt h in
      if visited.(s) then aux n (Heap.remove (d, s) h)
      else
        (visited.(s) <- true;
         aux (d+n) @@
         ListL.fold_left edges.(s) ~init:(Heap.remove (d,s) h)
           ~f:(fun h (t,d) ->
               if visited.(t) then h
               else Heap.add (d, t) h))
  in
  aux 0 @@ (Heap.empty |> Heap.add (0, 0))

let () =
  prim  |> Printf.printf "%d\n"
