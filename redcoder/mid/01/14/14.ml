open Batteries
open Tuple
module EnumL = Enum.Labels
module ListL = List.Labels

let dbg x = Printf.eprintf "[debug]%s\n" @@ dump x

let (++) n m = List.range n `To m
let (++^) n m = List.range n `To (pred m)

let scan fmt = Scanf.sscanf (read_line ()) fmt

let scan_lines n fmt f =
  if n = 0 then Enum.empty ()
  else
    List.map (fun _ -> scan fmt f) (List.range 1 `To n)
    |> List.enum

let scan_list cnv =
  String.split_on_char ' ' @@ read_line ()
  |> List.map cnv

let scan_array = Array.of_list % scan_list

let scan_matrix n m e conv =
  let arr = Array.make_matrix n m e in
  ListL.iter (List.range 0 `To (pred n))
    ~f:(fun i -> arr.(i) <- Array.of_list @@ scan_list conv);
  arr

let rec powerset e =
  match Enum.get e with
  | None -> Enum.singleton @@ Enum.empty ()
  | Some v ->
    let f = powerset e in
    let g = Enum.clone f in
    EnumL.map f ~f:(fun x -> let y = Enum.clone x in push y v; y)
    |>  Enum.append g

let intersection l =
  EnumL.filter ~f:(fun x -> EnumL.exists ~f:((=) x) @@ Enum.clone l)

let (n,k) = scan "%d %d" Tuple2.make
let m = scan_array Int.of_string

let () =
  let sets = powerset (0 --^ n) in
  EnumL.map sets ~f:(fun set ->
      if Enum.count @@ Enum.clone set = k then
        Array.fold_lefti (fun (h, acc) i a ->
            if Enum.exists ((=) i) (Enum.clone set) then
              if a > h then (a, acc)
              else (h+1, acc + h+1 -a)
            else
              (max h a, acc)) (0,0) m
        |> Tuple2.second
      else
        Int.max_num)
  |> Enum.reduce min
  |> Printf.printf "%d\n"
