open Batteries
module EnumL = Enum.Labels

let dbg x = Printf.eprintf "[debug]%s\n" @@ dump x; x

let scan fmt = Scanf.sscanf (read_line ()) fmt

let scan_lines n fmt f =
  if n = 0 then Enum.empty ()
  else
    List.map (fun _ -> scan fmt f) (List.range 1 `To n)
    |> List.enum

let scan_list cnv =
  String.split_on_char ' ' @@ read_line ()
  |> List.map cnv

let scan_array n m e conv =
  let arr = Array.make_matrix n m e in
  EnumL.iter (0 --^ n)
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

let intersection l =
  EnumL.filter ~f:(fun x -> EnumL.exists ~f:((=) x) @@ Enum.clone l)

let (n, m) = scan "%d %d" Tuple2.make
let ls = scan_lines m "%d %d" Tuple2.make

let () =
  let sets = powerset (1 -- n) in
  EnumL.map sets ~f:(fun set ->
      EnumL.map (Enum.clone set) ~f:(fun i ->
          EnumL.map (Enum.clone set) ~f:(fun j ->
              i = j  || EnumL.exists (Enum.clone ls)
                ~f:(fun v -> v = (i, j) || v = (j, i))))
      |> Enum.concat
      |> EnumL.for_all ~f:((=) true)
      |> fun b -> if b then Enum.count set else 1)
  |> Enum.reduce max
  |> max 1
  |> Printf.printf "%d\n"
