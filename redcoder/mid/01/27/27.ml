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

let m = scan "%d" id
let n = scan "%d" id
let mat = scan_matrix n m false (Bool.of_int % Int.of_string)

let () =
  ListL.map (0 ++^ n) ~f:(fun x ->
      ListL.map (0 ++^ m) ~f:(fun y ->
          if not mat.(x).(y) then 0
          else
            let visited = Array.create_matrix n m false in
            let rec aux x y d =
              visited.(x).(y) <- true;
              let ms = ListL.map [(-1,0);(1,0);(0,1);(0,-1)]
                  ~f:(fun (dx, dy) ->
                      let z, w = x+dx, y+dy in
                      if 0 <= z && z < n && 0 <= w && w < m
                         && not visited.(z).(w) && mat.(z).(w) then
                        aux z w @@ succ d
                      else d) in
              visited.(x).(y) <- false;
              List.max ms
            in
            aux x y 1))
  |> List.concat
  |> List.max
  |> Printf.printf "%d\n"
