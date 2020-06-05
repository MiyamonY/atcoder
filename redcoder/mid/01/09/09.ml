open Batteries

let dbg x = Printf.eprintf "[debug]%s\n" @@ dump x; x

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

let scan_lines n fmt f =
  List.Labels.map
    ~f:(fun _ -> scan fmt f)
    (List.range 1 `To n)
  |> List.enum


let scan_list cnv =
  read_line ()
  |> String.split_on_char ' '
  |> List.map cnv

let scan_array n m e conv =
  let arr = Array.make_matrix n m e in
  Enum.Labels.iter (0 --^ n)
    ~f:(fun i -> arr.(i) <- Array.of_list @@ scan_list conv);
  arr

let rec zip xs ys =
  match xs, ys with
  | [], _ -> []
  | _, [] -> []
  | x::xs, y::ys ->
    (x, y) :: zip xs ys

let m = scan "%d" identity
let ms = scan_lines m "%d %d" Tuple.Tuple2.make
let n = scan "%d" identity
let ns = scan_lines n "%d %d" Tuple.Tuple2.make

let () =
  let (x0, y0) = Enum.get_exn ms in
  Enum.Labels.map ns
    ~f:(fun (x, y) ->
        let dx = x-x0 in
        let dy = y-y0 in
        if Enum.Labels.for_all ms
            ~f:(fun (a,b) -> Enum.Labels.exists (Enum.clone ns) ~f:((=) (a+dx, b+dy)))
        then Some (dx, dy)
        else None)
  |> Enum.find Option.is_some
  |> Option.get
  |> fun (a,b) -> Printf.printf "%d %d\n" a b
