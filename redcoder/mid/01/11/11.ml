open Batteries
module EnumL = Enum.Labels
open Tuple

let dbg x = Printf.eprintf "[debug]%s\n" @@ dump x; x

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

let scan_lines n fmt f =
  List.Labels.map
    (List.range 1 `To n)
    ~f:(fun _ -> scan fmt f)
  |> List.enum

let scan_list cnv =
  let l = read_line ()
          |> String.split_on_char ' '
          |> List.map cnv in
  List.enum l

let scan_lists n cnv =
  let ll = List.Labels.map (List.range 1 `To n)
      ~f:(fun _ -> scan_list cnv) in
  List.enum ll

let scan_array n m e conv =
  let arr = Array.make_matrix n m e in
  EnumL.iter (0 --^ n)
    ~f:(fun i -> arr.(i) <- Array.of_enum @@ scan_list conv);
  arr

let rec powerset e =
  match Enum.get e with
  | None -> List.enum [Enum.empty ()]
  | Some v ->
    let f = powerset e in
    let g = Enum.clone f in
    EnumL.map f ~f:(fun x -> let y = Enum.clone x in push y v; y)
    |>  Enum.append g

let intersection l =
  EnumL.filter ~f:(fun x -> EnumL.exists ~f:((=) x) @@ Enum.clone l)

let (n, m) = scan "%d %d" Tuple2.make
let lights = scan_lists m Int.of_string
let onoff = scan_list Int.of_string

let () =
  let sets = powerset (1 -- n) in
  EnumL.map sets ~f:(fun set ->
      (* EnumL.map (Enum.combine (Enum.clone onoff) (Enum.clone lights)) *)
      EnumL.map (Enum.combine (Enum.clone onoff, Enum.clone lights))
        ~f:(fun (b, l) ->
            let n = Enum.count @@
              intersection set @@ Enum.skip 1 @@ Enum.clone l
            in n mod 2 = b)
      |> EnumL.for_all ~f:((=) true))
  |> EnumL.filter ~f:((=) true)
  |> Enum.count
  |> Printf.printf "%d\n"
