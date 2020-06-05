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

let (n,m,k,s) = scan "%d %d %d %d" Tuple4.make
let (p,q) = scan "%d %d" Tuple2.make
let ks = scan_lines k "%d" id
let edges =
  let e = Array.make n [] in
  ListL.iter (scan_lines m "%d %d" Tuple2.make) ~f:(fun (s,t) ->
      e.(pred s) <- (pred t) :: e.(pred s);
      e.(pred t) <- (pred s) :: e.(pred t)); e

let types = Array.make n 0

let rec wfs visited = function
  | [] -> ()
  | (v, n) ::rest ->
    if visited.(v) = true || n < 0 then wfs visited rest
    else
      ((if types.(v) <> 2 then types.(v) <-1);
       visited.(v) <- true;
       wfs visited @@ rest @ List.map (fun s -> (s, pred n)) edges.(v))

let rec solve dists h =
  if Heap.size h <> 0 then
    let (d, s) = Heap.find_min h in
    if dists.(s) > 0 then solve dists @@ Heap.del_min h
    else
      (dists.(s) <- d;
       solve dists @@
       ListL.fold_left edges.(s) ~init:Heap.(del_min h) ~f:(fun h t ->
           if dists.(t) <> -1 then h
           else
             match types.(t) with
             | 0 | 1 when t = pred n -> Heap.add (d, t) h
             | 0 -> Heap.add (d + p, t) h
             | 1 -> Heap.add (d + q, t) h
             | _ -> h))

let () =
  ListL.iter ks ~f:(fun k ->
      types.(pred k) <- 2;
      let visited = Array.make n false in
      wfs visited [(pred k, s)]);
  let dists = Array.make n (-1) in
  solve dists (Heap.empty |> Heap.add (0, 0));
  Printf.printf "%d\n" dists.(pred n)
