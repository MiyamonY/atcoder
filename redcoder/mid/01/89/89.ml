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

let n = scan "%d" id
let ls = scan_list ~sep:' ' Int.of_string

let () =
  ListL.fold_left
    (List.combine
       (List.take (pred @@ List.length ls) ls) (List.drop 1 ls)) ~init:[]
    ~f:(fun pred (a, b) ->
        if a = b then
          match pred with
          | [] -> [(b,b,1);(a,a,1)]
          | _ -> (b,b,1)::pred
        else
          match pred with
          | [] -> [(a,b,2)]
          | (c,_,n)::rest -> (c,b, succ n)::rest
      )
  |> Array.of_list % List.rev
  |> (fun arr -> Array.append arr @@
       ArrayL.mapi arr ~f:(fun i (a,b,n) ->
           (a, b,  n +
                   (if i > 0 then
                      let (_, b0, m) = arr.(pred i) in
                      if b0 = a then m
                      else 0
                    else 0) +
                   if succ i < Array.length arr then
                     let (a0, _, m) = arr.(succ i) in
                     if b = a0 then m
                     else 0
                   else 0)))
  |> ArrayL.map ~f:Tuple3.third
  |> Array.max
  |> Printf.printf "%d\n"
