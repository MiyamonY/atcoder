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

let () =
  let rec aux () =
    let (n, x) = scan "%d %d" Tuple.Tuple2.make in
    if not(n = 0 && x = 0) then
      (Enum.Labels.map (1 -- n)
         ~f:(fun i ->
             Enum.Labels.map (1 -- n)
               ~f:(fun j ->
                   Enum.Labels.map (1 -- n)
                     ~f:(fun k ->
                         if i < j && j < k then
                           if i + j +k = x then
                             1
                           else 0
                         else 0)))
       |> Enum.flatten |> Enum.flatten
       |> Enum.sum
       |> Printf.printf "%d\n";
       aux ()) in
  aux ()
