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

let dbg x = Printf.eprintf "[debug]%s\n" @@ dump x; x

let n = scan "%d" identity
let s = scan "%s" identity

let () =
  Enum.Labels.map (0 --^ 1000)
    ~f:(fun i ->
        let d = String.to_list @@ Printf.sprintf "%03d" i in
        let last = List.Labels.fold_left d
            ~init:0 ~f:(fun i c ->
                if i < n then
                  match String.Exceptionless.find_from s i @@ String.of_char c with
                  | Some i -> succ i
                  | None -> n+1
                else n+1)
        in
        if last < n+1 then 1 else 0)
  |> Enum.sum
  |> Printf.printf "%d\n"
