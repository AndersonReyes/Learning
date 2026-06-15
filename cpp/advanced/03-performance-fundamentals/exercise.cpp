#include "exercise.h"

#include <algorithm>

// --- DirectMappedCache ---------------------------------------------------------------------------

DirectMappedCache::DirectMappedCache(size_t numLines, size_t lineSize)
    : lines_(numLines), lineSize_(lineSize) {
    throw std::logic_error("not implemented");
}

bool DirectMappedCache::access(size_t address) {
    (void)address;
    throw std::logic_error("not implemented");
}

size_t DirectMappedCache::hits() const { throw std::logic_error("not implemented"); }

size_t DirectMappedCache::misses() const { throw std::logic_error("not implemented"); }

// --- Arena -----------------------------------------------------------------------------------------

Arena::Arena(size_t capacityBytes) : buffer_(capacityBytes) { throw std::logic_error("not implemented"); }

void* Arena::allocate(size_t size, size_t alignment) {
    (void)size;
    (void)alignment;
    throw std::logic_error("not implemented");
}

void Arena::reset() { throw std::logic_error("not implemented"); }

size_t Arena::bytesUsed() const { throw std::logic_error("not implemented"); }

size_t Arena::capacity() const { throw std::logic_error("not implemented"); }

// --- toSoA / totalKineticEnergy ------------------------------------------------------------------

ParticlesSoA toSoA(const std::vector<ParticleAoS>& particles) {
    (void)particles;
    throw std::logic_error("not implemented");
}

double totalKineticEnergy(const ParticlesSoA& particles) {
    (void)particles;
    throw std::logic_error("not implemented");
}

// --- transposeBlocked --------------------------------------------------------------------------

std::vector<std::vector<int>> transposeBlocked(const std::vector<std::vector<int>>& m,
                                                 size_t blockSize) {
    (void)m;
    (void)blockSize;
    throw std::logic_error("not implemented");
}

// --- multiplyBlocked ----------------------------------------------------------------------------

std::vector<std::vector<double>> multiplyBlocked(const std::vector<std::vector<double>>& A,
                                                   const std::vector<std::vector<double>>& B,
                                                   size_t blockSize) {
    (void)A;
    (void)B;
    (void)blockSize;
    throw std::logic_error("not implemented");
}
