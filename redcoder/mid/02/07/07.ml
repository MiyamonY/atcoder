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
let arr = Array.of_list @@ scan_lines n "%d %d" tuple2
let s = Hashtbl.create n

let () =
  ArrayL.iter arr ~f:(fun v -> Hashtbl.add s v true);
  ListL.map (0 ++^ n) ~f:(fun i ->
      ListL.map (i+1 ++^ n) ~f:(fun j ->
          let (x0,y0) = arr.(i) in
          let (x1,y1) = arr.(j) in
          let x = x1 - x0 in
          let y = y1 - y0 in
          if (Hashtbl.mem s (x0+y, y0-x)
              && Hashtbl.mem s (x0+y+x, y0-x+y))
          || (Hashtbl.mem s (x0-y, y0+x)
              && Hashtbl.mem s (x0-y+x, y0+x+y)) then x*x + y*y
          else 0) |> ListL.fold_left ~init:0 ~f:max)
  |> ListL.fold_left ~init:0 ~f:max
  |> Printf.printf "%d\n"
