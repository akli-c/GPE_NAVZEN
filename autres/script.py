# ce script permet de compter les valeurs uniques dans un fichier texte qui corespond au fichier SurfaceInfo.txt
# et d'afficher le nombre d'occurences de chaque valeur


from collections import Counter

def compter_valeurs_uniques(fichier, sortie=None):
    try:
        with open(fichier, 'r', encoding='utf-8') as f:
            lignes = [ligne.strip() for ligne in f if ligne.strip()]

        if not lignes:
            print("Le fichier est vide ou ne contient que des lignes vides.")
            return
        
        compteur = Counter(lignes)
        
        # Tri des lignes par ordre décroissant de fréquence
        valeurs_uniques = sorted(compteur.items(), key=lambda x: x[1], reverse=True)

        resultat = "\n".join(f"{ligne} → {occurences} fois" for ligne, occurences in valeurs_uniques)
        
        if sortie:
            with open(sortie, 'w', encoding='utf-8') as f_out:
                f_out.write(resultat)
            print(f"Résultats enregistrés dans {sortie}")
        else:
            print(resultat)

    except FileNotFoundError:
        print(f"Erreur : Le fichier '{fichier}' n'existe pas.")
    except Exception as e:
        print(f"Une erreur est survenue : {e}")

# Exemple d'utilisation
fichier_txt = "SurfaceInfo.txt"  # Remplace par le nom de ton fichier
fichier_sortie = "resultats.txt"  # Mettre à None si tu ne veux pas enregistrer

compter_valeurs_uniques(fichier_txt, fichier_sortie)
