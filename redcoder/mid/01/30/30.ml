open Batteries
module EnumL = Enum.Labels
module ListL = List.Labels
module ArrayL = Array.Labels

let dbg0 x = Printf.eprintf "[debug]%s\n" @@ dump x
let dbg1 x = dbg0 x; x

let id = identity

let atoi n = Char.chr (n + Char.code '0')
let (++) n m = List.range n `To m
let (++^) n m = if n < m then List.range n `To (pred m) else []
let (++~) c0 c1 = ListL.map ~f:Char.chr @@ ((Char.code c0) ++ (Char.code c1))

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

let (h, w, n) = scan "%d %d %d" Tuple.Tuple3.make
let g = Array.of_enum @@ scan_lines h "%s" id

let () =
  let poss = ListL.map ('S'::('1' ++~ atoi n)) ~f:(fun c ->
      let j = ref 0 in
      let i = ArrayL.findi g ~f:(fun l ->
          match String.Exceptionless.index l c with
          | None -> false
          | Some n -> j := n; true) in
      (i, !j)) in
  ListL.map (List.combine
               (List.take (pred @@ List.length poss) poss) @@ List.tl poss)
    ~f:(fun ((sx, sy),(gx, gy)) ->
        let dists = Array.make_matrix h w (-1) in
        let rec aux = function
          | [] -> ()
          | None :: rest -> aux rest
          | Some (x,y)::rest ->
            let d = dists.(x).(y) in
            aux @@ rest @
                   ListL.map [(1,0);(-1,0);(0,1);(0,-1)]
                     ~f:(fun (dx, dy) ->
                         let x, y = x + dx, y + dy in
                         if 0 <= x && x < h && 0<= y && y < w && dists.(x).(y) = -1 && String.get g.(x) y <> 'X' then
                           (dists.(x).(y) <- succ d;
                            Some (x,y))
                         else
                           None
                       )
        in
        dists.(sx).(sy) <- 0;
        aux [Some (sx, sy)];
        dists.(gx).(gy)
      )
  |> List.sum
  |> Printf.printf "%d\n"
