<?php

namespace App\Controller;

use App\Entity\User;
use App\Service\JwtTokenService;
use Doctrine\ORM\EntityManagerInterface;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\Routing\Annotation\Route;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\PasswordHasher\Hasher\UserPasswordHasherInterface;
use App\Repository\UserRepository;
use App\Enum\Level;

class AuthController extends AbstractController
{
    private UserPasswordHasherInterface $hasher;
    private JwtTokenService $jwtTokenService;
    private UserRepository $userRepository;

    public function __construct(
        UserPasswordHasherInterface $hasher, 
        JwtTokenService $jwtTokenService, 
        UserRepository $userRepository
    ) {
        $this->hasher = $hasher;
        $this->jwtTokenService = $jwtTokenService;
        $this->userRepository = $userRepository;
    }

    #[Route('/api/register', name: 'api_register', methods: ['POST'])]
    public function register(
        Request $request, 
        EntityManagerInterface $entityManager
    ): JsonResponse {
        $data = json_decode($request->getContent(), true);

        if (empty($data['email']) || empty($data['password']) || empty($data['username'])) {
            return $this->json(['message' => 'Email, username and password are required'], 400);
        }

        $existingUser = $this->userRepository->findOneBy(['username' => $data['username']]);
        $existingEmail = $this->userRepository->findOneBy(['email' => $data['email']]);
        if ($existingUser || $existingEmail) {
            return $this->json(['message' => 'Username or email is already in use'], 400);
        }

        $user = new User();
        $user->setEmail($data['email'])
             ->setUsername($data['username'])
             ->setPassword($this->hasher->hashPassword($user, $data['password']))
             ->setRoles(['ROLE_USER']);

        if (isset($data['level'])) {
            $enumLevel = Level::tryFrom($data['level']);
            if ($enumLevel) {
                $user->setLevel($enumLevel);
            } else {
                return $this->json(['message' => 'Invalid level provided. Allowed values: Débutant, Intermédiaire, Expert.'], 400);
            }
        }

        $entityManager->persist($user);
        $entityManager->flush();

        $token = $this->jwtTokenService->generateToken($user);

        return $this->json([
            'token' => $token,
            'message' => 'User created successfully'
        ], 201);
    }
}
