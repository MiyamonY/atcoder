open Batteries
module EnumL = Enum.Labels
module ListL = List.Labels
module ArrayL = Array.Labels

let dbg0 x = Printf.eprintf "[debug]%s\n" @@ dump x
let dbg1 x = dbg0 x; x

let id = identity

let (++) n m = List.range n `To m
let (++-) n m = if n >= m then List.range n `Downto m else []
let (++^) n m = if n < m then List.range n `To (pred m) else []

let scan fmt = Scanf.sscanf (read_line ()) fmt

let scan_lines n fmt f =
  if n = 0 then []
  else List.map (fun _ -> scan fmt f) (1 ++ n)

let scan_list sep cnv =
  let line = read_line () in
  (match sep with
   | None -> List.map String.of_char @@ String.to_list line
   | Some sep -> String.split_on_char sep line)
  |> List.map cnv

let scan_array ?sep cnv = Array.of_list @@ scan_list sep cnv

let scan_matrix n m e ?sep conv =
  let arr = Array.make_matrix n m e in
  ListL.iter (0 ++^ n)
    ~f:(fun i -> arr.(i) <- Array.of_list @@ scan_list sep conv);
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

let zip l m =
  let n = min (List.length l) (List.length m) in
  List.combine (List.take n l) (List.take n m)

let lower_bound n f =
  let rec aux l u =
    if u - l > 1 then
      let m = (l + u) / 2 in
      if f m then aux l m
      else aux m u
    else u in aux (-1) n

let between n x m = n <= x && x < m

let (n, q) = scan "%d %d" Tuple2.make
let edges = Array.make n []

let rec solve dists h =
  if Heap.size h > 0 then
    let (dist, s) = Heap.find_min h in
    if dists.(s) > -1 then solve dists (Heap.del_min h)
    else
      (dists.(s) <- dist;
       solve dists @@
       ListL.fold_left edges.(s) ~init:(Heap.del_min h) ~f:(fun h (t, d) ->
           if dists.(t) = -1 then
             Heap.add (dist+d, t) h
           else h))

let () =
  ListL.iter (0++^q) ~f:(fun _->
      let input = scan_array ~sep:' ' Int.of_string in
      if input.(0) = 1 then
        let (c,d,e) = input.(1), input.(2), input.(3) in
        edges.(pred c) <- (pred d, e)::edges.(pred c);
        edges.(pred d) <- (pred c, e)::edges.(pred d)
      else
        let (a,b) = input.(1), input.(2) in
        let h = Heap.empty |> Heap.add (0, pred a) in
        let dists = Array.make n (-1) in
        solve  dists h;
        Printf.printf "%d\n" dists.(pred b))
