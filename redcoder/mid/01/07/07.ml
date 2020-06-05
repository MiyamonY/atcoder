open Batteries
module EnumL = Enum.Labels

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
  EnumL.iter (0 --^ n)
    ~f:(fun i -> arr.(i) <- Array.of_list @@ scan_list conv);
  arr

let rec zip xs ys =
  match xs, ys with
  | [], _ -> []
  | _, [] -> []
  | x::xs, y::ys ->
    (x, y) :: zip xs ys

let n = scan "%d" identity
let ls = scan_lines n "%d %d" Tuple.Tuple2.make
let ht = Hashtbl.create n

let () =
  EnumL.iter (Enum.clone ls) ~f:(fun l -> Hashtbl.add ht l true);
  EnumL.map ls
    ~f:(fun (a, b) ->
        EnumL.map (Enum.clone ls)
          ~f:(fun (c,d) ->
              if (a,b) = (c,d) then
                0
              else
                let dx, dy = c - a, d - b in
                let z, w = (a+dy,b-dx), (c+dy, d-dx) in
                let t, s = (a-dy,b+dx), (c-dy, d+dx) in
                if (Hashtbl.mem ht z && Hashtbl.mem ht w) ||
                   (Hashtbl.mem ht t && Hashtbl.mem ht s) then
                  dx*dx+dy*dy
                else
                  0))
  |> Enum.concat
  |> Enum.reduce max
  |> Printf.printf "%d\n"
