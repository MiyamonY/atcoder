open Batteries
open Tuple
module EnumL = Enum.Labels
module ListL = List.Labels

let dbg x = Printf.eprintf "[debug]%s\n" @@ dump x

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
  List.enum % ListL.map ~f:List.enum @@ aux @@ List.of_enum l

let intersection l =
  EnumL.filter ~f:(fun x -> EnumL.exists ~f:((=) x) @@ Enum.clone l)

let n = scan "%d" id
let routes = Array.of_enum @@ scan_lines n "%f %f" Tuple2.make

let () =
  let perms = permutations (0 --^ n) in
  EnumL.map (Enum.clone perms) ~f:(fun perm ->
      let r = EnumL.map (Enum.clone perm) ~f:(Array.get routes) in
      junk perm;
      EnumL.map (Enum.combine (r, EnumL.map perm ~f:(Array.get routes)))
        ~f:(fun ((x0, y0), (x1, y1)) ->
            Float.sqrt ((Float.pow (x0 -. x1) 2.0) +.  (Float.pow (y0 -. y1) 2.0)))
      |> Enum.fsum
    )
  |> Enum.fsum
  |> (fun f -> f /. (Float.of_int @@ Enum.count perms))
  |> Printf.printf "%f\n"
