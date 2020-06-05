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

let s = scan "%s" identity

let () =
  let n = String.length s in
  Enum.Labels.map (0 --^ n)
    ~f:(fun i ->
        Enum.Labels.map (1 -- n) ~f:(fun j ->
            if i < j then
              let sub = String.slice ~first:i ~last:j s  in
              if Enum.for_all (String.contains "AGTC") @@ String.enum sub then
                j - i
              else 0
            else
              0))
  |> Enum.concat
  |> Enum.reduce max
  |> Printf.printf "%d\n"
