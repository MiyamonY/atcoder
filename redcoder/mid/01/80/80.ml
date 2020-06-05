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

let (h,w,k,v) = scan "%d %d %d %d" Tuple4.make
let mat = scan_matrix h w 0 ~sep:' ' Int.of_string

let () =
  let acc = Array.make_matrix (succ h) (succ w) 0 in
  ListL.iter (1 ++ h) ~f:(fun i ->
      ListL.fold_left (1 ++ w) ~init:0 ~f:(fun p j ->
          let p = p + mat.(pred i).(pred j) in
          acc.(i).(j) <- p + acc.(pred i).(j); p
        ) |> ignore);
  let ans = ref 0 in
  ListL.iter (1 ++ w) ~f:(fun x0 ->
      ListL.iter (x0 ++ w) ~f:(fun x1 ->
          ListL.iter (1 ++ h) ~f:(fun y0 ->
              ListL.iter (y0 ++ h) ~f:(fun y1 ->
                  ans := max !ans
                      (let n = (y1 - y0+1) * (x1-x0+1)in
                       let d = acc.(y1).(x1) - acc.(y1).(pred x0)
                               - acc.(pred y0).(x1)
                               + acc.(pred y0).(pred x0) in
                       if d + n*k <= v then n
                       else 0)))));
  Printf.printf "%d\n" !ans
