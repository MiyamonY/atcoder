open Batteries
open Tuple
module EnumL = Enum.Labels
module ListL = List.Labels

let dbg x = Printf.eprintf "[debug]%s\n" @@ dump x; x

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

let (r,c) = scan "%d %d" Tuple2.make
let m = scan_matrix r c 0 Int.of_string

let () =
  let reverse i = Array.Labels.modify m.(i)
      ~f:(fun v -> if v = 1 then 0 else 1) in
  let count () =
    ListL.map (0 ++^ c) ~f:(fun c ->
        ListL.map (0 ++^ r) ~f:(fun r ->
            m.(r).(c))
        |> List.sum
        |> fun a -> max a (r-a))
    |> List.sum
  in
  let sets = powerset (0 --^ r) in
  EnumL.map sets ~f:(fun set ->
      EnumL.iter (Enum.clone set) ~f:reverse;
      let x = count () in
      EnumL.iter (Enum.clone set) ~f:reverse;
      x)
  |> Enum.reduce max
  |> Printf.printf "%d\n"
