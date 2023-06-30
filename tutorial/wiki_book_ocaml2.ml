

(* https://fr.wikibooks.org/wiki/Objective_Caml/Bases#Exercices_2 *)

max;;

let appliquer_operation_postfixe =
  fun x ->
  fun y ->
  fun o ->
   o x y ;;

let multiplie n f = 
  fun x -> n * ( f x ) ;;