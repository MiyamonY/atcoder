open Batteries
open BatPrintf
open BatBigarray

let scan fmt f =
  BatScanf.sscanf (read_line ()) fmt f

let scan_list cnv =
  read_line ()
  |> BatString.split_on_char ' '
  |> List.map cnv

let rec zip xs ys =
  match xs, ys with
  | [], _ -> []
  | _, [] -> []
  | x::xs, y::ys ->
    (x, y) :: zip xs ys

let n = scan "%s" identity
let k = scan "%d" identity

let () =
  let len = String.length n in
  let dp = Array3.create Int C_layout 2 (len+1) len in
  Array3.set dp 0 0 0 1;
  if len < k then
    printf "%d\n" 0
  else
    let open BatEnum in
    iter (fun i ->
        let d = int_of_string @@ String.of_char @@ String.get n (i-1) in
        iter (fun j ->
            Array3.set dp 1 i j @@ (Array3.get dp 1 (i - 1) j) * 9 +
                                   (if j > 0 then
                                      Array3.get dp 1 (i - 1) (j - 1)
                                    else 0) +
                                   if d > 0 then
                                     (d - 1) * (Array3.get dp 0 (i - 1) j) + (if j > 0 then Array3.get dp 0 (i - 1) (j - 1) else 0)
                                   else
                                     0;
            Array3.set dp 0 i j @@
            if d > 0 then
              Array3.get dp 0 (i-1) j
            else if j > 0 then
              Array3.get dp 0 (i-1) (j-1)
            else
              0
          ) (0 --^ len);
      ) (1 -- len);
    printf "%d\n" @@ Array3.get dp 1 len (len - k) + Array3.get dp 0 len (len - k)
