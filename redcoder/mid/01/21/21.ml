open Batteries
module EnumL = Enum.Labels
module ListL = List.Labels
module ArrayL = Array.Labels

let dbg0 x = Printf.eprintf "[debug]%s\n" @@ dump x
let dbg1 x = dbg0 x; x

let id = identity

let (++) n m = List.range n `To m
let (++^) n m = if n < m then List.range n `To (pred m) else []

let scan fmt = Scanf.sscanf (read_line ()) fmt

let scan_list cnv =
  String.split_on_char ' ' @@ read_line ()
  |> List.map cnv

let scan_lines n fmt f =
  if n = 0 then Enum.empty ()
  else
    List.map (fun _ -> scan fmt f) (1 ++ n)
    |> List.enum

let scan_array n fmt f =
  Array.init n
    (fun _ -> Scanf.sscanf (read_line ()) fmt f)

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

let lower_bound n f =
  let rec aux l u =
    if u - l > 1 then
      let m = (l + u) / 2 in
      if f m then aux l m
      else aux m u
    else u in aux (-1) n

let n = scan "%d" id
let hs = scan_array n "%d %d" Tuple.Tuple2.make

let () =
  let rec aux l u f =
    if u - l > 1 then
      let mid = (u + l) / 2 in
      if f mid then aux l mid f
      else aux mid u f
    else u
  in
  aux 0 Int.max_num (fun v ->
      if not @@ ArrayL.for_all hs ~f:(fun (h,_) -> h <= v) then
        false
      else
        let a = ArrayL.map hs ~f:(fun (h, s) -> (v-h)/s) in
        Array.sort Int.compare a;
        Array.fold_lefti (fun acc i n -> acc && i <= n) true a)
  |> Printf.printf "%d\n"
