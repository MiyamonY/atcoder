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

let n = scan "%d" id
let mat = scan_matrix ~sep:' ' n n 0 Int.of_string
let mat2 = Array.make_matrix n n true

let check () =
  ListL.map (0 ++^ n) ~f:(fun k ->
      ListL.map (0 ++^ n) ~f:(fun i ->
          ListL.map (0 ++^ n) ~f:(fun j ->
              if i <> k && k <> j &&
                 mat.(i).(j) = mat.(i).(k) + mat.(k).(j) then
                mat2.(i).(j) <- false;
              if mat.(i).(j) > mat.(i).(k) + mat.(k).(j) then -1
              else mat.(i).(j)
            ) |> List.min
        ) |> List.min
    ) |> List.min
  |> (fun v -> v = -1)

let () =
  if check () then Printf.printf "-1\n"
  else
    (ArrayL.mapi mat2 ~f:(fun i a ->
         ArrayL.mapi a ~f:(fun j v ->
             if v then mat.(i).(j)
             else 0
           ) |> Array.sum
       ) |> Array.sum
     |> (fun v -> Printf.printf "%d\n" (v/2)))
