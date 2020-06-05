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

let scan_lines n fmt f =
  if n = 0 then []
  else
    List.map (fun _ -> scan fmt f) (List.range 1 `To n)

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

let lower_bound n f =
  let rec aux l u =
    if u - l > 1 then
      let m = (l + u) / 2 in
      if f m then aux l m
      else aux m u
    else u in aux (-1) n

let (n, m) = scan "%d %d" Tuple.Tuple2.make
let p = 0:: scan_lines n "%d" id

let () =
  let len = List.length p in
  let a = Array.make (len * len) 0  in
  ListL.iteri p ~f:(fun i x ->
      ListL.iteri p ~f:(fun j y -> a.(i*len+j) <- x+y));
  Array.sort Int.compare a;
  ArrayL.map a ~f:(fun v ->
      let i = lower_bound (Array.length a) (fun i ->
          m <= v + a.(i)) in
      if i = 0 then 0
      else v + a.(pred i))
  |> Array.max
  |> Printf.printf "%d\n"
