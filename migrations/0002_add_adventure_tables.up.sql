-- Create AdventureNode table
CREATE TABLE IF NOT EXISTS AdventureNodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    node_type VARCHAR(50) NOT NULL,
    body TEXT NOT NULL
);

-- Create Character table
CREATE TABLE IF NOT EXISTS Characters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    current_node_id UUID,
    FOREIGN KEY (current_node_id) REFERENCES AdventureNodes(id)
);

-- Create AdventureSpot table
CREATE TABLE IF NOT EXISTS AdventureSpots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL
);

-- Create AdventureOption table
CREATE TABLE IF NOT EXISTS AdventureOptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    body TEXT NOT NULL,
    next_node_id UUID,
    FOREIGN KEY (next_node_id) REFERENCES AdventureNodes(id)
);

-- Create AdventureSpotAdventureNode join table
CREATE TABLE IF NOT EXISTS AdventureSpotsAdventureNodes (
    adventure_spot_id UUID NOT NULL,
    adventure_node_id UUID NOT NULL,
    PRIMARY KEY (adventure_spot_id, adventure_node_id),
    FOREIGN KEY (adventure_spot_id) REFERENCES AdventureSpots(id),
    FOREIGN KEY (adventure_node_id) REFERENCES AdventureNodes(id)
);

-- Create AdventureNodeAdventureOption join table
CREATE TABLE IF NOT EXISTS AdventureNodesAdventureOptions (
    adventure_node_id UUID NOT NULL,
    adventure_option_id UUID NOT NULL,
    PRIMARY KEY (adventure_node_id, adventure_option_id),
    FOREIGN KEY (adventure_node_id) REFERENCES AdventureNodes(id),
    FOREIGN KEY (adventure_option_id) REFERENCES AdventureOptions(id)
);
