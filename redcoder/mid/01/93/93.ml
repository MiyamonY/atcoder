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

let scan_list ?sep cnv =
  let line = read_line () in
  (match sep with
   | None -> List.map String.of_char @@ String.to_list line
   | Some sep -> String.split_on_char sep line)
  |> List.map cnv

let scan_array ?sep cnv = Array.of_list @@ scan_list ?sep cnv

let scan_matrix n m e ?sep conv =
  let arr = Array.make_matrix n m e in
  ListL.iter (0 ++^ n)
    ~f:(fun i -> arr.(i) <- Array.of_list @@ scan_list ?sep conv);
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


let (h,w,k) = scan "%d %d %d" Tuple3.make
let mat = scan_matrix h w 0 Int.of_string

exception Found

let shrink mat =
  ListL.iter (0++^ w) ~f:(fun j ->
      ListL.iter ((pred h) ++- 0) ~f:(fun k ->
          if mat.(k).(j) = 0 then
            try ListL.iter ((pred k) ++- 0) ~f:(fun l ->
                if mat.(l).(j) <> 0 then
                  (mat.(k).(j) <- mat.(l).(j);
                   mat.(l).(j) <- 0;
                   raise Found)) with
            | Found -> ()))

let find line =
  Array.to_list line
  |>  List.fold_lefti (fun pred i w ->
      match pred with
      | [] -> [(w, i, 1)]
      | (v,s,n)::rest when v = w ->
        (v,s,succ n) :: rest
      | _  ->
        (w,i,1)::pred
    ) []
  |> List.filter (fun (_,_,n) -> n >= k)

let extinct m =
  ListL.map (0 ++^ h) ~f:(fun i ->
      let line = m.(i) in
      match find line with
      | [] -> 0
      | _ as ls ->
        (ListL.map ls ~f:(fun (v, s, n) ->
             ListL.iter (0 ++^ n) ~f:(fun i -> line.(s+i) <- 0);
             v*n)
         |> List.sum))
  |> List.sum

let () =
  ListL.map (0++^h) ~f:(fun i ->
      ListL.map (0++^w) ~f:(fun j ->
          (* Printf.printf "start %d %d\n" i j; *)
          let m = Array.make h [| |] in
          ListL.iter (0++^h) ~f:(fun i -> m.(i) <- Array.copy mat.(i));
          m.(i).(j) <- 0;
          let v = ListL.map (0++h) ~f:(fun n ->
              shrink m;
              Int.pow 2 n * extinct m) |> List.sum in
          (* Array.print ~first:"" ~sep:"" ~last:"" (Array.print ~first:"" ~sep:" " ~last:"\n" Int.print) stdout m; *)
          v))
  |> List.concat
  |> List.max
  |> Printf.printf "%d\n"
