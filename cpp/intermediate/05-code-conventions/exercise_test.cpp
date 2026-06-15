#include <string>
#include <vector>

#include "exercise.h"
#include "../../testing.h"

TEST(HasStatusSubsetIncludingVacuousTruth) {
    CHECK(hasStatus(Status::Active | Status::Done, Status::Active));
    CHECK(!hasStatus(Status::Active, Status::Active | Status::Done));
    CHECK(hasStatus(Status::Active, Status::None));  // vacuous truth
    CHECK(hasStatus(Status::None, Status::None));
}

TEST(SortByPriorityIsStableDescending) {
    std::vector<Task> tasks = {
        {"A", Priority::Medium, Status::None},
        {"B", Priority::High, Status::None},
        {"C", Priority::Low, Status::None},
        {"D", Priority::High, Status::None},
    };

    std::vector<Task> sorted = sortByPriority(tasks);
    std::vector<std::string> names;
    for (const auto& t : sorted) names.push_back(t.name);
    CHECK(names == (std::vector<std::string>{"B", "D", "A", "C"}));

    std::vector<Task> allTied = {
        {"X", Priority::Low, Status::None},
        {"Y", Priority::Low, Status::None},
        {"Z", Priority::High, Status::None},
    };
    std::vector<Task> sortedTied = sortByPriority(allTied);
    std::vector<std::string> tiedNames;
    for (const auto& t : sortedTied) tiedNames.push_back(t.name);
    CHECK(tiedNames == (std::vector<std::string>{"Z", "X", "Y"}));
}

TEST(CountByPriorityCountsEachLevel) {
    std::vector<Task> tasks = {
        {"A", Priority::Medium, Status::None},
        {"B", Priority::High, Status::None},
        {"C", Priority::Low, Status::None},
        {"D", Priority::High, Status::None},
    };
    CHECK_EQ(countByPriority(tasks), (PriorityCount{1, 1, 2}));
    CHECK_EQ(countByPriority({}), (PriorityCount{0, 0, 0}));
}

TEST(NamesWithStatusFiltersByBitmaskSubset) {
    std::vector<Task> tasks = {
        {"A", Priority::Medium, Status::Active},
        {"B", Priority::Low, Status::Active | Status::Blocked},
        {"C", Priority::High, Status::Done},
        {"D", Priority::High, Status::Active | Status::Done},
    };

    CHECK(namesWithStatus(tasks, Status::Active) == (std::vector<std::string>{"A", "B", "D"}));
    CHECK(namesWithStatus(tasks, Status::Active | Status::Blocked) ==
          (std::vector<std::string>{"B"}));
    CHECK(namesWithStatus(tasks, Status::Blocked | Status::Done) == std::vector<std::string>{});
    // Status::None matches every task (vacuous truth).
    CHECK(namesWithStatus(tasks, Status::None) ==
          (std::vector<std::string>{"A", "B", "C", "D"}));
}

TEST(HighestPriorityActiveBreaksTiesByOrder) {
    std::vector<Task> tasks = {
        {"A", Priority::Medium, Status::Active},
        {"B", Priority::Low, Status::Active | Status::Blocked},
        {"C", Priority::High, Status::Done},  // not Active -- excluded
        {"D", Priority::High, Status::Active | Status::Done},
    };
    CHECK_EQ(highestPriorityActive(tasks).name, std::string("D"));

    std::vector<Task> tied = {
        {"E", Priority::High, Status::Active},
        {"F", Priority::High, Status::Active},
    };
    CHECK_EQ(highestPriorityActive(tied).name, std::string("E"));
}

TEST_MAIN()
