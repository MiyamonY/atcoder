open Batteries
open BatPrintf

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

let scan_list cnv =
  read_line ()
  |> String.split_on_char ' '
  |> List.map cnv

let scan_listn n cnv =
  (0 --^ n) |> Enum.map (fun _ -> (cnv % read_line) ())

let bsearch_ge arr a =
  let l = Array.length arr in
  match Array.bsearch Int.ord arr a with
  | `All_lower -> l
  | `All_bigger -> 0
  | `Just_after n -> l - n -1
  | `At n -> l - n
  | `Empty -> 0

let rec zip xs ys =
  match xs, ys with
  | [], _ -> []
  | _, [] -> []
  | x::xs, y::ys ->
    (x, y) :: zip xs ys

let square x = x * x

let dbg n = Printf.printf "%s\n" @@ dump n; n

module Comb = struct
  let prime = 1_000_000_000+7

  let ( * ) a b = a * b mod prime
  let (-) a b = (a+prime - b) mod prime

  let rec pow n m =
    if m = 0 then 1
    else if m mod 2 = 0 then pow (n*n) (m/2)
    else n * pow (n*n) (m/2)

  let fact n =
    let rec aux m ret =
      if m = 0 then ret
      else aux (m-1) @@ m*ret in
    aux n 1

  let comb n k =
    let x = Enum.fold ( * ) 1 (n-k+1 -- n) in
    x * (pow (fact k) (prime - 2))
end

let (n, a, b) = scan "%d %d %d" Tuple.Tuple3.make


let () =
  let open Comb in
  pow 2 n
  |> (fun x -> (-) x 1)
  |> (fun x -> (-) x @@ comb n a)
  |> (fun x -> (-) x @@ comb n b)
  |> printf "%d\n"
