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

let dbg n = Printf.eprintf "%s\n" @@ dump n; n

module CombMemo = struct
  let prime = 1_000_000_000+7
  let size = 2*1_00_000+1

  let memo = let arr = Array.make size None in
    arr.(0) <- Some 1;
    arr

  let ( *% ) a b = a * b mod prime
  let (-%) a b = (a+prime - b) mod prime
  let (+%) a b = (a + b) mod prime

  let rec pow n m =
    if m = 0 then 1
    else if m mod 2 = 0 then pow (n*%n) (m/2)
    else n *% pow (n*%n) (m/2)

  let rec fact n =
    match memo.(n) with
    | None ->
      let m = n *% fact (n-1) in
      memo.(n) <- Some m;
      m
    | Some a -> a

  let comb n k =
    if n <= 0 || k <= 0 then 1
    else
      fact n *% (pow (fact k) (prime-2)) *% (pow (fact (n - k)) (prime-2))
end

let (n, k) = scan "%d %d" Tuple.Tuple2.make

let () =
  let open CombMemo in
  (0 -- if k >= n-1 then n-1 else k)
  |> Enum.fold (fun s m -> s +% (comb n m *% comb (n-1) (n-m-1))) 0
  |> printf "%d\n"
