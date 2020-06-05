open Batteries
module EnumL = Enum.Labels
module ListL = List.Labels

let dbg0 x = Printf.eprintf "[debug]%s\n" @@ dump x
let dbg1 x = Printf.eprintf "[debug]%s\n" @@ dump x; x

let id = identity

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

let scan_array cnv = Array.of_list @@ scan_list cnv

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

let permutations l =
  let rec aux l =
    let rec interleave x = function
      | [] -> [[x]]
      | (hd::tl) as lst ->
        (x::lst) ::
        (ListL.map ~f:(List.cons hd)  @@ interleave x tl)
    in
    match l with
    | [] -> [[]]
    | hd::tl -> List.concat @@ List.map (interleave hd) @@ aux tl in
  let l = List.sort (List.compare Int.compare) @@ aux @@ List.of_enum l in
  List.enum % ListL.map ~f:List.enum @@ l

let intersection l =
  EnumL.filter ~f:(fun x -> EnumL.exists ~f:((=) x) @@ Enum.clone l)

let d = scan "%d" id
let n = scan "%d" id
let m = scan "%d" id
let ss = scan_lines (n-1) "%d" id
let ds = scan_lines m "%d" id

let () =
  Enum.push ss 0;
  let ss = Array.of_enum ss in
  let ss = Array.insert ss d @@ Array.length ss in
  Array.sort Int.compare ss;
  EnumL.map ds ~f:(fun d ->
      match Array.bsearch Int.ord ss d with
      | `At _ -> 0
      | `Just_after i ->
        min (abs (ss.(i) - d)) (abs (ss.(succ i) - d))
      | _ -> 0)
  |> Enum.reduce (+)
  |> Printf.printf "%d\n"
