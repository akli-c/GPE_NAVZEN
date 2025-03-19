<?php

namespace App\Service;

use Symfony\Component\Security\Core\User\UserInterface;
use Lexik\Bundle\JWTAuthenticationBundle\Services\JWTTokenManagerInterface;

class JwtTokenService
{

    private $jwtManager;

    public function __construct(JWTTokenManagerInterface $jwtManager)
    {
        $this->jwtManager = $jwtManager;
    }

    public function generateToken(UserInterface $user): string
    {
        $test = $this->jwtManager->create($user);
        return $test;
    }

   

    // /**
    //  * Génère un token avec une configuration donnée
    //  */
    // public function generateToken(User $user, Configuration $config, int $ttl): string
    // {
    //     $now = new DateTimeImmutable();
    //     return $config->builder()
    //         ->identifiedBy(bin2hex(random_bytes(10))) // Identifiant unique du token
    //         ->issuedAt($now)
    //         ->expiresAt($now->modify("+{$ttl} seconds")) // Expiration dynamique
    //         ->withClaim('id', $user->getId())
    //         ->withClaim('email', $user->getEmail())
    //         ->withClaim('roles', $user->getRoles())
    //         ->getToken($config->signer(), $config->signingKey())
    //         ->toString();
    // }

    // /**
    //  * Vérifie et décode un token
    //  */
    // public function validateToken(string $token): ?UnencryptedToken
    // {
    //     $config = $this->accessConfig;

    //     try {
    //         $parsedToken = $config->parser()->parse($token);
    //         $constraints = $config->validationConstraints();

    //         if (!$config->validator()->validate($parsedToken, ...$constraints)) {
    //             return null; // Token invalide
    //         }

    //         return $parsedToken;
    //     } catch (\Exception $e) {
    //         return null; // Erreur lors du parsing du token
    //     }
    // }
}
