<?php

namespace App\Security;

use App\Entity\User;
use App\Repository\UserRepository;
use App\Service\JwtTokenService;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Security\Core\Authentication\Token\TokenInterface;
use Symfony\Component\Security\Core\Exception\AuthenticationException;
use Symfony\Component\Security\Http\Authenticator\AbstractAuthenticator;
use Symfony\Component\Security\Http\Authenticator\Passport\Badge\UserBadge;
use Symfony\Component\Security\Http\Authenticator\Passport\Credentials\PasswordCredentials;
use Symfony\Component\Security\Http\Authenticator\Passport\Passport;
use Symfony\Component\Security\Http\Authenticator\Passport\PassportInterface;

class LoginAuthenticator extends AbstractAuthenticator
{
    private JwtTokenService $jwtTokenService;
    private UserRepository $userRepository;

    public function __construct(JwtTokenService $jwtTokenService, UserRepository $userRepository)
    {
        $this->jwtTokenService = $jwtTokenService;
        $this->userRepository = $userRepository;
    }

    public function supports(Request $request): ?bool
    {
        return $request->getPathInfo() === '/api/login' && $request->isMethod('POST');
    }

    public function authenticate(Request $request): Passport
    {
        $data = json_decode($request->getContent(), true);

        // login can be email or username
        $login = $data['login'] ?? '';
        $password = $data['password'] ?? '';

        if (empty($login) || empty($password)) {
            throw new AuthenticationException('Login and password are required.');
        }

        return new Passport(
            new UserBadge($login, function(string $userIdentifier): ?User {
                // First, try to find the user by email.
                $user = $this->userRepository->findOneBy(['email' => $userIdentifier]);
                if (!$user) {
                    // If not found by email, try to find by username.
                    $user = $this->userRepository->findOneBy(['username' => $userIdentifier]);
                }
                if (!$user) {
                    throw new AuthenticationException('User not found.');
                }
                return $user;
            }),
            new PasswordCredentials($password)
        );
    }

    public function onAuthenticationSuccess(Request $request, TokenInterface $token, string $firewallName): ?Response
    {
        /** @var User $user */
        $user = $token->getUser();
        $jwtToken = $this->jwtTokenService->generateToken($user);

        return new JsonResponse([
            'token' => $jwtToken,
        ]);
    }

    public function onAuthenticationFailure(Request $request, AuthenticationException $exception): ?Response
    {
        return new JsonResponse([
            'message' => 'Authentication failed: ' . $exception->getMessage(),
        ], Response::HTTP_UNAUTHORIZED);
    }
}
