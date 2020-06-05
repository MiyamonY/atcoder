open Batteries

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

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

let dbg x = Printf.printf "%s\n" @@ dump x; x

let (a,b,c,x,y) = scan "%d %d %d %d %d" Tuple.Tuple5.make

let () =
  let n = min x y in
  let m = max x y in
  List.min [a*x+b*y; c*2*n+a*(x-n)+b*(y-n);c*2*m]
  |> Printf.printf "%d\n"
