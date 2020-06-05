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
  if n = 0 then Enum.empty ()
  else
    List.map (fun _ -> scan fmt f) (1 ++ n)
    |> List.enum

let scan_list cnv =
  String.split_on_char ' ' @@ read_line ()
  |> List.map cnv

let scan_array cnv = Array.of_list @@ scan_list cnv

let scan_matrix n m e conv =
  let arr = Array.make_matrix n m e in
  ListL.iter (0 ++^ n)
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

let (w, h) = scan "%d %d" Tuple.Tuple2.make
let mat = scan_matrix h w false @@ Bool.of_int % Int.of_string
let kinds = Array.make_matrix h w true

let neighbors = [|[(-1,0);(-1,1);(0, -1);(0,1);(1,0);(1,1)];
                  [(-1,-1);(-1,0);(0,-1);(0,1);(1,-1);(1, 0)]|]

let rec bfs visited = function
  | [] -> ()
  | (i,j) :: rest ->
    kinds.(i).(j) <- false;
    visited.(i).(j) <- true;
    bfs visited @@ ListL.fold_left ~init:rest neighbors.(i mod 2)
      ~f:(fun l (dx, dy) ->
          let x, y = i + dx, j + dy in
          if 0 <= x && x < h && 0 <= y && y < w
             && not mat.(x).(y) && not visited.(x).(y)
          then (x,y)::l
          else l)

let () =
  let visited  = Array.make_matrix h w false in
  ArrayL.iteri mat ~f:(fun i line ->
      ArrayL.iteri line ~f:(fun j v ->
          if i = 0 || i = h-1 || j = 0 || j = w -1 then
            if not v && not visited.(i).(j) then
              bfs visited [(i, j)]));
  ArrayL.mapi kinds ~f:(fun i line ->
      ArrayL.mapi line ~f:(fun j v ->
          if v then
            ListL.map neighbors.(i mod 2) ~f:(fun (dx,dy) ->
                let x, y = i + dx, j + dy in
                if 0<= x && x < h && 0 <= y && y < w  then
                  if kinds.(x).(y) then 0 else 1
                else 1)
            |> List.sum
          else 0)
      |> Array.sum)
  |> Array.sum
  |> Printf.printf "%d\n"
